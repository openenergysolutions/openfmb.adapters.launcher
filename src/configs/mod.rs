#![allow(dead_code)]

// SPDX-FileCopyrightText: 2022 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use tokio::process::Child;

const ICCP_CLIENT: &str = "iccp-client";
const ICCP_SERVER: &str = "iccp-server";

const IEC61850_CLIENT: &str = "IEC61850-client";
const IEC61850_SERVER: &str = "IEC61850-server";

const DNP3_MASTER: &str = "dnp3-master";
const DNP3_OUTSTATION: &str = "dnp3-outstation";

const MODBUS_MASTER: &str = "modbus-master";
const MODBUS_OUTSTATION: &str = "modbus-outstation";

const OCPP_1_6: &str = "ocpp";

const OES_PLUG: &str = "oes-plug";

const PUB_SUB_BRIDGE: &str = "pub-sub-bridge";

const HISTORIAN: &str = "historian";

/// Launcher configuration structure
#[derive(Debug, Serialize, Deserialize)]
pub struct LauncherConfig {
    /// Launcher
    #[serde(rename = "launcher")]
    pub launcher: Launcher,

    /// List of adapters
    #[serde(rename = "adapters")]
    pub adapters: Vec<Adapter>,
}

/// Launcher structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Launcher {
    // Log level. Possible values are Trace, Debug, Info, Error.  Default is Debug
    pub log_level: Option<String>,
}

/// Adapter structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Adapter {
    /// Name of the adapter
    pub name: String,

    /// Type of the adapter (iccp-client, IEC61850, etc.)
    #[serde(rename = "type")]
    pub typ: String,

    /// Full path to the adapter's configuration file
    pub config: String,

    ///  OS-assigned process identifier associated with this adapter child process while it is still running
    pub pid: Option<u32>,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub child: Option<Child>,
}

impl LauncherConfig {
    /// Get the executable file name for the specified adapter
    pub fn get_executable_name(adapter_type: &str) -> Option<(String, (String, String))> {
        match adapter_type {
            PUB_SUB_BRIDGE | HISTORIAN | DNP3_MASTER | DNP3_OUTSTATION | MODBUS_MASTER
            | MODBUS_OUTSTATION => Some(("openfmb-adapter".into(), ("".into(), "".into()))),
            ICCP_CLIENT | ICCP_SERVER => Some((
                "iccp-adapter".into(),
                ("LD_LIBRARY_PATH".into(), "/usr/local/lib/iccp".into()),
            )),
            IEC61850_CLIENT | IEC61850_SERVER => Some((
                "iec61850-adapter".into(),
                ("LD_LIBRARY_PATH".into(), "/usr/local/lib/iec61850".into()),
            )),
            OCPP_1_6 => Some(("ocpp-adapter".into(), ("".into(), "".into()))),
            OES_PLUG => Some(("udp-adapter".into(), ("".into(), "".into()))),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ConfigError(String);

impl std::error::Error for ConfigError {}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ConfigError {
    pub fn new(s: String) -> ConfigError {
        ConfigError(s)
    }
}
