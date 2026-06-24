//! Persistent ROS2 daemon subprocess (rclpy) for in-process I/O.
//!
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::Mutex;

static DAEMON: Mutex<Option<Ros2Daemon>> = Mutex::new(None);

struct Ros2Daemon {
    child: Child,
    stdin: ChildStdin,
    reader: BufReader<std::process::ChildStdout>,
}

impl Ros2Daemon {
    fn start() -> Result<Self, String> {
        // Description:
        //     Start.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: Result<Self, String>
        //         Return value from `start`.
        //
        // Example:
        //     let result = spanda_transport_ros2::daemon::start();

        let script = daemon_script_path()?;
        let python = python_cmd().ok_or_else(|| "python3 not found for ROS2 daemon".to_string())?;
        let mut child = Command::new(&python)
            .arg(&script)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("failed to start ROS2 daemon: {e}"))?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| "daemon stdin unavailable".to_string())?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| "daemon stdout unavailable".to_string())?;
        Ok(Self {
            child,
            stdin,
            reader: BufReader::new(stdout),
        })
    }

    fn request(&mut self, op: &str, args: &[String]) -> bool {
        // Description:

        //     Request.

        //

        // Inputs:

        //     &mut self: value

        //         Caller-supplied &mut self.

        //     op: &str

        //         Caller-supplied op.

        //     args: &[String]

        //         Caller-supplied args.

        //

        // Outputs:

        //     result: bool

        //         Return value from `request`.

        //

        // Example:

        //     let result = spanda_transport_ros2::daemon::request(&mut self, op, args);

        let payload = serde_json::json!({ "op": op, "args": args });
        let line = match serde_json::to_string(&payload) {
            Ok(text) => text,
            Err(_) => return false,
        };
        if writeln!(self.stdin, "{line}").is_err() {
            return false;
        }
        if self.stdin.flush().is_err() {
            return false;
        }
        let mut response = String::new();
        if self.reader.read_line(&mut response).is_err() {
            return false;
        }
        serde_json::from_str::<serde_json::Value>(&response)
            .ok()
            .and_then(|value| value.get("ok").and_then(|ok| ok.as_bool()))
            .unwrap_or(false)
    }
}

fn python_cmd() -> Option<String> {
    // Description:

    //     Python cmd.

    //

    // Inputs:

    //     None.

    //

    // Outputs:

    //     result: Option<String>

    //         Return value from `python_cmd`.

    //

    // Example:

    //     let result = spanda_transport_ros2::daemon::python_cmd();

    for cmd in ["python3", "python"] {
        if Command::new(cmd)
            .arg("-c")
            .arg("import sys")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
        {
            return Some(cmd.to_string());
        }
    }
    None
}

pub fn python_available() -> bool {
    // Description:

    //     Python available.

    //

    // Inputs:

    //     None.

    //

    // Outputs:

    //     result: bool

    //         Return value from `python_available`.

    //

    // Example:

    //     let result = spanda_transport_ros2::daemon::python_available();

    python_cmd().is_some()
}

pub fn daemon_script_path() -> Result<PathBuf, String> {
    // Description:

    //     Daemon script path.

    //

    // Inputs:

    //     None.

    //

    // Outputs:

    //     result: Result<PathBuf, String>

    //         Return value from `daemon_script_path`.

    //

    // Example:

    //     let result = spanda_transport_ros2::daemon::daemon_script_path();

    if let Ok(path) = std::env::var("SPANDA_ROS2_DAEMON_SCRIPT") {
        let path = PathBuf::from(path);
        if path.is_file() {
            return Ok(path);
        }
    }
    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        let path = PathBuf::from(manifest)
            .join("../../scripts/spanda_ros2_daemon.py")
            .canonicalize()
            .ok();
        if let Some(path) = path {
            if path.is_file() {
                return Ok(path);
            }
        }
    }
    let path = PathBuf::from("scripts/spanda_ros2_daemon.py");
    if path.is_file() {
        return Ok(path);
    }
    Err("spanda_ros2_daemon.py not found".into())
}

fn with_daemon<F>(f: F) -> bool
where
    F: FnOnce(&mut Ros2Daemon) -> bool,
{
    // Description:

    //     With daemon.

    //

    // Inputs:

    //     f: F

    //         Caller-supplied f.

    //

    // Outputs:

    //     result: bool where F: FnOnce(&mut Ros2Daemon) -> bool,

    //         Return value from `with_daemon`.

    //

    // Example:

    //     let result = spanda_transport_ros2::daemon::with_daemon(f);

    if !python_available() {
        return false;
    }
    let mut guard = match DAEMON.lock() {
        Ok(guard) => guard,
        Err(_) => return false,
    };
    if guard.is_none() {
        match Ros2Daemon::start() {
            Ok(daemon) => *guard = Some(daemon),
            Err(_) => return false,
        }
    }
    let daemon = guard.as_mut().expect("daemon");
    if daemon.child.try_wait().ok().flatten().is_some() {
        match Ros2Daemon::start() {
            Ok(restarted) => *daemon = restarted,
            Err(_) => {
                *guard = None;
                return false;
            }
        }
    }
    f(guard.as_mut().expect("daemon"))
}

pub fn daemon_publish(topic: &str, payload: &str) -> bool {
    // Description:

    //     Daemon publish.

    //

    // Inputs:

    //     opic: &str

    //         Caller-supplied opic.

    //     payload: &str

    //         Caller-supplied payload.

    //

    // Outputs:

    //     result: bool

    //         Return value from `daemon_publish`.

    //

    // Example:

    //     let result = spanda_transport_ros2::daemon::daemon_publish(opic, payload);

    with_daemon(|daemon| daemon.request("publish", &[topic.to_string(), payload.to_string()]))
}

pub fn daemon_subscribe(topic: &str) -> bool {
    // Description:

    //     Daemon subscribe.

    //

    // Inputs:

    //     opic: &str

    //         Caller-supplied opic.

    //

    // Outputs:

    //     result: bool

    //         Return value from `daemon_subscribe`.

    //

    // Example:

    //     let result = spanda_transport_ros2::daemon::daemon_subscribe(opic);

    with_daemon(|daemon| daemon.request("subscribe", &[topic.to_string()]))
}

pub fn daemon_service_call(service: &str, service_type: &str, request: &str) -> bool {
    // Description:

    //     Daemon service call.

    //

    // Inputs:

    //     service: &str

    //         Caller-supplied service.

    //     service_type: &str

    //         Caller-supplied service type.

    //     request: &str

    //         Caller-supplied request.

    //

    // Outputs:

    //     result: bool

    //         Return value from `daemon_service_call`.

    //

    // Example:

    //     let result = spanda_transport_ros2::daemon::daemon_service_call(service, service_type, reques);

    with_daemon(|daemon| {
        daemon.request(
            "service_call",
            &[
                service.to_string(),
                service_type.to_string(),
                request.to_string(),
            ],
        )
    })
}
