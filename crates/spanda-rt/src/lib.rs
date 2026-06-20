//! Native runtime linked with LLVM-generated Spanda programs.
//!
//! Exposes a stable C ABI (`spanda_rt_*`) for actuator calls, logging, and
//! emergency stop. The interpreter/simulator remains authoritative for now;
//! this crate is the link target for Milestone 2 LLVM codegen.

use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Mutex;

static EVENTS: Mutex<Vec<String>> = Mutex::new(Vec::new());

#[no_mangle]
pub extern "C" fn spanda_rt_drive(actuator: *const c_char, linear: f64, angular: f64) {
    let name = ptr_to_str(actuator);
    log_event(format!("drive:{name}:{linear}:{angular}"));
}

#[no_mangle]
pub extern "C" fn spanda_rt_stop(actuator: *const c_char) {
    let name = ptr_to_str(actuator);
    log_event(format!("stop:{name}"));
}

#[no_mangle]
pub extern "C" fn spanda_rt_emergency_stop() {
    log_event("emergency_stop".into());
}

#[no_mangle]
pub extern "C" fn spanda_rt_log_i32(tag: *const c_char, value: i32) {
    let name = ptr_to_str(tag);
    log_event(format!("log:{name}:{value}"));
}

#[no_mangle]
pub extern "C" fn spanda_rt_publish(topic: *const c_char, payload: *const c_char) {
    let topic = ptr_to_str(topic);
    let payload = ptr_to_str(payload);
    log_event(format!("publish:{topic}:{payload}"));
}

#[no_mangle]
pub extern "C" fn spanda_rt_subscribe(topic: *const c_char) {
    let topic = ptr_to_str(topic);
    log_event(format!("subscribe:{topic}"));
}

#[no_mangle]
pub extern "C" fn spanda_rt_loop_delay_ms(millis: u64) {
    log_event(format!("loop_delay:{millis}"));
    std::thread::sleep(std::time::Duration::from_millis(millis));
}

/// Test helper: drain and return recorded runtime events.
pub fn take_events() -> Vec<String> {
    EVENTS.lock().unwrap().drain(..).collect()
}

fn log_event(msg: String) {
    if let Ok(mut events) = EVENTS.lock() {
        events.push(msg);
    }
}

fn ptr_to_str(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return "<null>".into();
    }
    unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
}

fn events_since(start: usize) -> Vec<String> {
    EVENTS.lock().unwrap()[start..].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn c_abi_records_actuator_calls() {
        let start = EVENTS.lock().unwrap().len();
        let wheels = CString::new("wheels").unwrap();
        spanda_rt_drive(wheels.as_ptr(), 0.5, 0.1);
        spanda_rt_stop(wheels.as_ptr());
        let events = events_since(start);
        assert!(events.iter().any(|event| event.starts_with("drive:wheels:")));
        assert!(events.iter().any(|event| event == "stop:wheels"));
    }

    #[test]
    fn c_abi_records_publish_and_loop() {
        let start = EVENTS.lock().unwrap().len();
        let topic = CString::new("/status").unwrap();
        let payload = CString::new("ok").unwrap();
        spanda_rt_publish(topic.as_ptr(), payload.as_ptr());
        spanda_rt_loop_delay_ms(1);
        let events = events_since(start);
        assert!(events.iter().any(|event| event == "publish:/status:ok"));
        assert!(events.iter().any(|event| event == "loop_delay:1"));
    }

    #[test]
    fn c_abi_records_subscribe() {
        let start = EVENTS.lock().unwrap().len();
        let topic = CString::new("/cmd").unwrap();
        spanda_rt_subscribe(topic.as_ptr());
        let events = events_since(start);
        assert!(events.iter().any(|event| event == "subscribe:/cmd"));
    }
}
