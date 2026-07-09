//! GPS, IMU, and camera sensor pipelines with hub stubs and env-gated live bridges.
//!
use spanda_ast::nodes::UnitKind;
use spanda_runtime::value::RuntimeValue;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

static SENSOR_HUB: OnceLock<Mutex<SensorHub>> = OnceLock::new();

fn hub() -> &'static Mutex<SensorHub> {
    SENSOR_HUB.get_or_init(|| Mutex::new(SensorHub::default()))
}

/// In-memory GPS/IMU/camera samples for simulation and package stubs.
#[derive(Default)]
pub struct SensorHub {
    gps: HashMap<String, GpsFix>,
    imu: HashMap<String, ImuSample>,
    camera: HashMap<String, CameraSample>,
}

/// GPS/GNSS fix in decimal degrees and metres.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GpsFix {
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
    pub heading: f64,
}

/// IMU orientation and acceleration sample.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImuSample {
    pub roll: f64,
    pub pitch: f64,
    pub yaw: f64,
    pub ax: f64,
    pub ay: f64,
    pub az: f64,
}

/// Camera frame metadata for fusion and motion scoring.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraSample {
    pub width: u32,
    pub height: u32,
    pub motion_score: f64,
}

impl SensorHub {
    pub fn seed_demo(&mut self) {
        self.gps.insert(
            "gps".into(),
            GpsFix {
                lat: 37.7749,
                lon: -122.4194,
                alt: 12.0,
                heading: 90.0,
            },
        );
        self.imu.insert(
            "imu".into(),
            ImuSample {
                roll: 0.02,
                pitch: -0.01,
                yaw: 1.57,
                ax: 0.0,
                ay: 0.0,
                az: 9.81,
            },
        );
        self.camera.insert(
            "camera".into(),
            CameraSample {
                width: 1280,
                height: 720,
                motion_score: 0.15,
            },
        );
    }

    pub fn read_gps(&self, sensor_id: &str) -> GpsFix {
        let key = if sensor_id.is_empty() { "gps" } else { sensor_id };
        self.gps.get(key).copied().unwrap_or(GpsFix {
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
            heading: 0.0,
        })
    }

    pub fn read_imu(&self, sensor_id: &str) -> ImuSample {
        let key = if sensor_id.is_empty() { "imu" } else { sensor_id };
        self.imu.get(key).copied().unwrap_or(ImuSample {
            roll: 0.0,
            pitch: 0.0,
            yaw: 0.0,
            ax: 0.0,
            ay: 0.0,
            az: 9.81,
        })
    }

    pub fn read_camera(&self, sensor_id: &str) -> CameraSample {
        let key = if sensor_id.is_empty() {
            "camera"
        } else {
            sensor_id
        };
        self.camera.get(key).copied().unwrap_or(CameraSample {
            width: 640,
            height: 480,
            motion_score: 0.0,
        })
    }
}

/// Seed demo GPS/IMU/camera readings for installed sensor packages.
pub fn seed_sensor_demos() {
    hub().lock().unwrap().seed_demo();
}

/// Read a GPS fix (live bridge when enabled, otherwise hub stub).
pub fn read_gps_fix(sensor_id: &str) -> GpsFix {
    if let Some((lat, lon, alt, heading)) = crate::iot_live::read_gps_fix_live(sensor_id) {
        return GpsFix {
            lat,
            lon,
            alt,
            heading,
        };
    }
    hub().lock().unwrap().read_gps(sensor_id)
}

/// Read an IMU sample (live bridge when enabled, otherwise hub stub).
pub fn read_imu_sample(sensor_id: &str) -> ImuSample {
    if let Some((roll, pitch, yaw, ax, ay, az)) = crate::iot_live::read_imu_sample_live(sensor_id) {
        return ImuSample {
            roll,
            pitch,
            yaw,
            ax,
            ay,
            az,
        };
    }
    hub().lock().unwrap().read_imu(sensor_id)
}

/// Read camera metadata (live bridge when enabled, otherwise hub stub).
pub fn read_camera_sample(sensor_id: &str) -> CameraSample {
    if let Some((width, height, motion_score)) =
        crate::iot_live::read_camera_sample_live(sensor_id)
    {
        return CameraSample {
            width,
            height,
            motion_score,
        };
    }
    hub().lock().unwrap().read_camera(sensor_id)
}

/// GPS fix as a runtime object for package dispatch and positioning providers.
pub fn read_gps_as_runtime_value(sensor_id: &str) -> RuntimeValue {
    gps_runtime_value(read_gps_fix(sensor_id))
}

/// IMU sample as a runtime object for package dispatch.
pub fn read_imu_as_runtime_value(sensor_id: &str) -> RuntimeValue {
    imu_runtime_value(read_imu_sample(sensor_id))
}

/// Camera frame metadata as a runtime object for package dispatch.
pub fn read_camera_as_runtime_value(sensor_id: &str) -> RuntimeValue {
    camera_runtime_value(read_camera_sample(sensor_id))
}

fn gps_runtime_value(fix: GpsFix) -> RuntimeValue {
    RuntimeValue::Object {
        type_name: "GpsFix".into(),
        fields: HashMap::from([
            (
                "lat".into(),
                RuntimeValue::Number {
                    value: fix.lat,
                    unit: UnitKind::None,
                },
            ),
            (
                "lon".into(),
                RuntimeValue::Number {
                    value: fix.lon,
                    unit: UnitKind::None,
                },
            ),
            (
                "altitude".into(),
                RuntimeValue::Number {
                    value: fix.alt,
                    unit: UnitKind::M,
                },
            ),
            (
                "heading".into(),
                RuntimeValue::Number {
                    value: fix.heading,
                    unit: UnitKind::Deg,
                },
            ),
            (
                "fix_quality".into(),
                RuntimeValue::Number {
                    value: 1.0,
                    unit: UnitKind::None,
                },
            ),
        ]),
    }
}

fn imu_runtime_value(sample: ImuSample) -> RuntimeValue {
    RuntimeValue::Object {
        type_name: "IMUReading".into(),
        fields: HashMap::from([
            (
                "roll".into(),
                RuntimeValue::Number {
                    value: sample.roll,
                    unit: UnitKind::Rad,
                },
            ),
            (
                "pitch".into(),
                RuntimeValue::Number {
                    value: sample.pitch,
                    unit: UnitKind::Rad,
                },
            ),
            (
                "yaw".into(),
                RuntimeValue::Number {
                    value: sample.yaw,
                    unit: UnitKind::Rad,
                },
            ),
            (
                "ax".into(),
                RuntimeValue::Number {
                    value: sample.ax,
                    unit: UnitKind::None,
                },
            ),
            (
                "ay".into(),
                RuntimeValue::Number {
                    value: sample.ay,
                    unit: UnitKind::None,
                },
            ),
            (
                "az".into(),
                RuntimeValue::Number {
                    value: sample.az,
                    unit: UnitKind::None,
                },
            ),
        ]),
    }
}

fn camera_runtime_value(sample: CameraSample) -> RuntimeValue {
    RuntimeValue::Object {
        type_name: "CameraFrame".into(),
        fields: HashMap::from([
            (
                "width".into(),
                RuntimeValue::Number {
                    value: sample.width as f64,
                    unit: UnitKind::None,
                },
            ),
            (
                "height".into(),
                RuntimeValue::Number {
                    value: sample.height as f64,
                    unit: UnitKind::None,
                },
            ),
            (
                "motion_score".into(),
                RuntimeValue::Number {
                    value: sample.motion_score,
                    unit: UnitKind::None,
                },
            ),
        ]),
    }
}

/// Overlay interpreter sensor reads with live GPS/IMU/camera pipeline values when enabled.
pub fn overlay_sensor_reading(
    sensor_type: &str,
    sensor_name: &str,
    simulated: &RuntimeValue,
) -> RuntimeValue {
    match sensor_type {
        "GPS" | "GNSS" => gps_runtime_value(read_gps_fix(sensor_name)),
        "IMU" | "BoschBNO055" | "LSM9DS1" => imu_runtime_value(read_imu_sample(sensor_name)),
        "Camera" | "VisionCamera" | "RGBCamera" | "DepthCamera" => {
            camera_runtime_value(read_camera_sample(sensor_name))
        }
        _ => simulated.clone(),
    }
}

/// Live fusion samples for cognitive sensory fusion (GPS lat/lon, IMU yaw, camera motion).
pub fn live_fusion_sensor_readings(_entity_id: &str) -> Vec<(String, f64, f64)> {
    if !crate::iot_live::live_fusion_sensors_enabled() {
        return Vec::new();
    }
    let gps = read_gps_fix("gps");
    let imu = read_imu_sample("imu");
    let camera = read_camera_sample("camera");
    vec![
        ("gps_lat".into(), gps.lat, 0.95),
        ("gps_lon".into(), gps.lon, 0.95),
        ("imu_yaw".into(), imu.yaw, 0.9),
        ("camera_motion".into(), camera.motion_score, 0.8),
    ]
}
