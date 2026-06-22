# IoT Support (Package-First)

IoT integrations live in official packages. Core defines generic contracts; packages implement protocols.

## Core contracts

| Trait | Purpose |
|-------|---------|
| `IoTDeviceProvider` | Device lifecycle and identity |
| `TelemetryProvider` | Sensor reading ingestion |
| `CommandProvider` | Remote command dispatch |
| `DeviceShadowProvider` | Desired/reported state sync |

## Core types

`IoTDevice`, `DeviceShadow`, `Telemetry`, `Command`, `SensorReading`, `ActuatorCommand`

## Official packages

| Package | Protocol |
|---------|----------|
| `spanda-iot-core` | Base contracts and types |
| `spanda-mqtt` | MQTT pub/sub |
| `spanda-ble` | Bluetooth LE |
| `spanda-wifi` | WiFi connectivity |
| `spanda-cellular` | LTE/cellular |
| `spanda-opcua` | OPC-UA (stub) |
| `spanda-modbus` | Modbus (stub) |
| `spanda-canbus` | CAN bus (stub) |
| `spanda-zigbee` | Zigbee (stub) |
| `spanda-lora` | LoRa (stub) |
| `spanda-matter` | Matter (stub) |

## Example

```spanda
device TemperatureSensor {
    protocol: mqtt;
    topic: "/factory/temp";
}
```

Install packages via `spanda add spanda-mqtt`.
