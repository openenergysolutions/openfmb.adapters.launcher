// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

//! An app that launch OpenFMB adapters based on the configuration

mod configs;

use configs::{ConfigError, LauncherConfig};
use ctrlc;

use std::result::Result;
use std::sync::mpsc::{channel, Receiver};
use std::{env, fs};
use tokio::process::Command;

/// Supervisor to launch/relaunch plugins and adapters
#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // read args
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: ./adapters-launcher -c config.yaml");
        return;
    }

    let contents =
        fs::read_to_string(&args[2]).expect(&format!("ERROR:: Unable to read file at {}", args[2]));

    let mut config = serde_yaml::from_str::<LauncherConfig>(&contents).unwrap();

    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    match launch(&mut config, rx) {
        Ok(_) => {
            log::info!("All adapters exited gracefully!");
        }
        Err(e) => log::error!("Failed to launch any of the adapters. {}", e),
    }
}

fn launch(config: &mut LauncherConfig, rx: Receiver<()>) -> Result<(), ConfigError> {
    if config.adapters.len() == 0 {
        return Err(ConfigError::new(
            "No adapters found.  Nothing to be launched!".to_string(),
        ));
    }

    match config.adapters.len() {
        0 => {
            return Err(ConfigError::new(
                "No adapters found.  Nothing to be launched!".to_string(),
            ))
        }
        _ => {
            loop {
                if let Ok(_) = rx.try_recv() {
                    break;
                }

                for mut a in config.adapters.iter_mut() {
                    if let Some(child) = &mut a.child {
                        match child.try_wait() {
                            Ok(status) => {
                                if let Some(status) = status {
                                    log::info!(
                                        "Process {} exited with status: {:?}",
                                        a.name,
                                        status
                                    );
                                    a.child = None;
                                }
                            }
                            Err(e) => {
                                log::warn!("Failed to get process status for {} ({})", a.name, e)
                            }
                        }
                    } else {
                        // Get exe based on adapter type
                        match LauncherConfig::get_executable_name(&a.typ) {
                            Some((exe, (k, v))) => {
                                match Command::new(&exe)
                                    .args(&["-c", &a.config])
                                    .env(&k, &v)
                                    .spawn()
                                {
                                    Ok(child) => a.child = Some(child),
                                    Err(e) => {
                                        log::error!("Failed to start process {} ({})", a.typ, e)
                                    }
                                }
                            }
                            None => {
                                return Err(ConfigError::new(format!(
                                    "Unable to find executable file for adapter {}",
                                    a.name
                                )));
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
