# OpenFMB Adapters Launcher

An app that launches OpenFMB adapters based on its configurations

Currently, the following protocol plug-ins are supported:

- dnp3-master
- dnp3-outstation
- modbus-master
- modbus-outstation
- pub-sub-bridge (bridging between pub/sub protocols such as NATS, MQTT, ZENOH, DDS)*
- oes-plug (fictituos OES Plug that speaks UDP)
- ocpp-adapter (OCPP 1.6)*
- iccp-client (IEC60870-6)*
- iccp-server (IEC60870-6)*
- IEC61850-client*
- IEC61850-server*

(*) Required appropriate licensing.  Please contact Open Enery Solutions, Inc

## Launcher Configuration

```bash
---
launcher:
  log_level: Debug
adapters: 
  - name: iccp-client1
    type: iccp-client
    config: /tmp/adapter.yaml
  - name: dnp3-master1
    type: openfmb-adapter    
    config: /tmp/adapter2.yaml  
...
```
