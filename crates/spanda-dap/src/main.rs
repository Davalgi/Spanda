//! main support for Spanda.
//!
use serde_json::{json, Value};
use spanda_debug::{DebugOptions, DebugPause, DebugSession};
use spanda_driver::{DebugMachine, DebugStepKind};
use spanda_error::SpandaError;
use std::cell::RefCell;
use std::collections::HashSet;
use std::io::{self, BufRead, Write};

thread_local! {
    static DEBUG_MACHINE: RefCell<Option<DebugMachine>> = const { RefCell::new(None) };
}

fn read_message(reader: &mut dyn BufRead) -> io::Result<Option<Value>> {
    // Description:
    //     Read message.
    //
    // Inputs:
    //     reader: &mut dyn BufRead
    //         Caller-supplied reader.
    //
    // Outputs:
    //     result: io::Result<Option<Value>>
    //         Return value from `read_message`.
    //
    // Example:
    //     let result = spanda_dap::main::read_message(reader);

    // Create mutable line for accumulating results.
    let mut line = String::new();
    let mut content_length = 0usize;

    // Run the loop body until it exits.
    loop {
        line.clear();

        // Take the branch when read line equals 0.
        if reader.read_line(&mut line)? == 0 {
            return Ok(None);
        }

        // Emit output when strip prefix provides a rest.
        if let Some(rest) = line.strip_prefix("Content-Length:") {
            content_length = rest.trim().parse().unwrap_or(0);
        } else if line.trim().is_empty() && content_length > 0 {
            break;
        }
    }
    let mut body = vec![0u8; content_length];
    reader.read_exact(&mut body)?;
    Ok(Some(serde_json::from_slice(&body)?))
}

fn write_message(writer: &mut dyn Write, msg: &Value) -> io::Result<()> {
    // Description:
    //     Write message.
    //
    // Inputs:
    //     writer: &mut dyn Write
    //         Caller-supplied writer.
    //     sg: &Value
    //         Caller-supplied sg.
    //
    // Outputs:
    //     result: io::Result<()>
    //         Return value from `write_message`.
    //
    // Example:
    //     let result = spanda_dap::main::write_message(writer, sg);

    // Hold the function body for execution.
    let body = serde_json::to_string(msg)?;
    write!(writer, "Content-Length: {}\r\n\r\n{}", body.len(), body)?;
    writer.flush()
}

fn respond(writer: &mut dyn Write, req: &Value, body: Value) -> io::Result<()> {
    // Description:
    //     Respond.
    //
    // Inputs:
    //     writer: &mut dyn Write
    //         Caller-supplied writer.
    //     req: &Value
    //         Caller-supplied req.
    //     body: Value
    //         Caller-supplied body.
    //
    // Outputs:
    //     result: io::Result<()>
    //         Return value from `respond`.
    //
    // Example:
    //     let result = spanda_dap::main::respond(writer, req, body);

    // Produce write message as the result.
    write_message(
        writer,
        &json!({
            "seq": req.get("seq").cloned().unwrap_or(json!(0)),
            "type": "response",
            "request_seq": req.get("seq"),
            "success": true,
            "command": req.get("command"),
            "body": body,
        }),
    )
}

fn step_kind(command: &str) -> DebugStepKind {
    // Description:
    //     Step kind.
    //
    // Inputs:
    //     command: &str
    //         Caller-supplied command.
    //
    // Outputs:
    //     result: DebugStepKind
    //         Return value from `step_kind`.
    //
    // Example:
    //     let result = spanda_dap::main::step_kind(command);

    // Match on command and handle each case.
    match command {
        "stepIn" => DebugStepKind::StepIn,
        "stepOut" => DebugStepKind::StepOut,
        "next" => DebugStepKind::StepOver,
        _ => DebugStepKind::Continue,
    }
}

fn with_machine<F, R>(
    source: &str,
    source_path: Option<&str>,
    breakpoints: &HashSet<u32>,
    f: F,
) -> Result<R, SpandaError>
where
    F: FnOnce(&mut DebugMachine) -> Result<R, SpandaError>,
{
    // Description:

    //     With machine.

    //

    // Inputs:

    //     source: &str

    //         Caller-supplied source.

    //     source_path: Option<&str>

    //         Caller-supplied source path.

    //     breakpoints: &HashSet<u32>

    //         Caller-supplied breakpoints.

    //     f: F

    //         Caller-supplied f.

    //

    // Outputs:

    //     result: Result<R, SpandaError> where F: FnOnce(&mut DebugMachine) -> Result<R, SpandaError>,

    //         Return value from `with_machine`.

    //

    // Example:

    //     let result = spanda_dap::main::with_machine(source, source_path, breakpoints, f);
    DEBUG_MACHINE.with(|cell| {
        let mut slot = cell.borrow_mut();

        // Take this path when slot.is none().
        if slot.is_none() {
            *slot = Some(DebugMachine::start(
                source,
                DebugOptions {
                    breakpoints: breakpoints.clone(),
                    step: false,
                    source_path: source_path.map(String::from),
                },
            )?);
        }
        let machine = slot.as_mut().expect("debug machine");
        f(machine)
    })
}

pub fn serve(
    source: &str,
    source_path: Option<&str>,
    reader: &mut dyn BufRead,
    writer: &mut dyn Write,
) -> io::Result<()> {
    // Description:
    //     Serve.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //     source_path: Option<&str>
    //         Caller-supplied source path.
    //     reader: &mut dyn BufRead
    //         Caller-supplied reader.
    //     writer: &mut dyn Write
    //         Caller-supplied writer.
    //
    // Outputs:
    //     result: io::Result<()>
    //         Return value from `serve`.
    //
    // Example:
    //     let result = spanda_dap::main::serve(source, source_path, reader, writer);

    // Create mutable breakpoints for accumulating results.
    let mut breakpoints: HashSet<u32> = HashSet::new();
    let mut running = false;

    // Repeat while let Some(req) = read message(reader)?.
    while let Some(req) = read_message(reader)? {
        let command = req
            .get("command")
            .and_then(|v| v.as_str())
            .unwrap_or_default();

        // Match on command and handle each case.
        match command {
            "initialize" => {
                respond(
                    writer,
                    &req,
                    json!({
                        "capabilities": {
                            "supportsConfigurationDoneRequest": true,
                            "supportsSetVariable": true,
                            "supportsStepBack": false,
                            "supportsRestartRequest": false,
                            "supportsStepIn": true,
                            "supportsStepOut": true,
                        }
                    }),
                )?;
            }
            "launch" => {
                running = true;
                DEBUG_MACHINE.with(|cell| *cell.borrow_mut() = None);
                respond(writer, &req, json!({}))?;
            }
            "setBreakpoints" => {
                breakpoints.clear();

                // Emit output when req provides a bps.
                if let Some(bps) = req
                    .pointer("/arguments/breakpoints")
                    .and_then(|v| v.as_array())
                {
                    // Iterate over bps.
                    for bp in bps {
                        // Emit output when as u64 provides a line.
                        if let Some(line) = bp.get("line").and_then(|l| l.as_u64()) {
                            breakpoints.insert(line as u32);
                        }
                    }
                }
                DEBUG_MACHINE.with(|cell| *cell.borrow_mut() = None);
                let verified: Vec<Value> = breakpoints
                    .iter()
                    .map(|line| json!({ "verified": true, "line": line }))
                    .collect();
                respond(writer, &req, json!({ "breakpoints": verified }))?;
            }
            "configurationDone" => {
                respond(writer, &req, json!({}))?;
            }
            "continue" | "next" | "stepIn" | "stepOut" | "pause" => {
                // Take this path when running.
                if running {
                    let kind = step_kind(command);
                    let session = with_machine(source, source_path, &breakpoints, |machine| {
                        machine.run_until_pause(kind)
                    })
                    .unwrap_or_else(|e: SpandaError| DebugSession {
                        pauses: vec![DebugPause {
                            line: 1,
                            reason: e.to_string(),
                            variables: Default::default(),
                        }],
                    });

                    // Process each pause.
                    for pause in session.pauses {
                        if pause.reason.starts_with("health_")
                            || pause.reason.starts_with("kill_switch")
                        {
                            write_message(
                                writer,
                                &json!({
                                    "type": "event",
                                    "event": "output",
                                    "body": {
                                        "category": "spanda-health",
                                        "output": pause.reason,
                                    }
                                }),
                            )?;
                        }
                        write_message(
                            writer,
                            &json!({
                                "type": "event",
                                "event": "stopped",
                                "body": {
                                    "reason": pause.reason,
                                    "threadId": 1,
                                    "text": pause.reason,
                                    "line": pause.line,
                                }
                            }),
                        )?;
                    }
                }
                respond(writer, &req, json!({ "allThreadsContinued": true }))?;
            }
            "setVariable" => {
                let name = req
                    .pointer("/arguments/name")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let value = req
                    .pointer("/arguments/value")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let ok = with_machine(source, source_path, &breakpoints, |machine| {
                    machine.set_variable(name, value)
                })
                .is_ok();
                respond(
                    writer,
                    &req,
                    json!({
                        "value": value,
                        "type": "string",
                        "variablesReference": 0,
                        "namedVariables": if ok { 1 } else { 0 },
                    }),
                )?;
            }
            "threads" => {
                respond(
                    writer,
                    &req,
                    json!({ "threads": [{ "id": 1, "name": "spanda-main" }] }),
                )?;
            }
            "stackTrace" => {
                let frames = with_machine(source, source_path, &breakpoints, |machine| {
                    let source = machine.source_path().map(|path| json!({ "path": path }));
                    Ok(machine
                        .stack_trace()
                        .into_iter()
                        .enumerate()
                        .map(|(idx, (name, line))| {
                            let mut frame = json!({
                                "id": idx + 1,
                                "name": name,
                                "line": line,
                                "column": 1,
                            });

                            // Emit output when source provides a source.
                            if let Some(source) = &source {
                                frame["source"] = source.clone();
                            }
                            frame
                        })
                        .collect::<Vec<_>>())
                })
                .unwrap_or_else(|_| vec![json!({"id": 1, "name": "main", "line": 1, "column": 1})]);
                respond(
                    writer,
                    &req,
                    json!({
                        "stackFrames": frames,
                        "totalFrames": frames.len(),
                    }),
                )?;
            }
            "scopes" => {
                let frame_id = req
                    .pointer("/arguments/frameId")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1) as usize;
                respond(
                    writer,
                    &req,
                    json!({
                        "scopes": [{
                            "name": "Locals",
                            "variablesReference": frame_id,
                            "expensive": false,
                        }]
                    }),
                )?;
            }
            "variables" => {
                let frame_id = req
                    .pointer("/arguments/variablesReference")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1) as usize;
                let variables: Vec<Value> = DEBUG_MACHINE.with(|cell| {
                    cell.borrow()
                        .as_ref()
                        .map(|machine| {
                            machine
                                .frame_variables(frame_id)
                                .iter()
                                .map(|(name, value)| {
                                    json!({
                                        "name": name,
                                        "value": value,
                                        "type": "String",
                                        "variablesReference": 0,
                                        "evaluateName": name,
                                    })
                                })
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default()
                });
                respond(writer, &req, json!({ "variables": variables }))?;
            }
            "disconnect" => {
                DEBUG_MACHINE.with(|cell| *cell.borrow_mut() = None);
                respond(writer, &req, json!({}))?;
                break;
            }
            _ => {
                respond(writer, &req, json!({}))?;
            }
        }
    }
    Ok(())
}

fn main() {
    // Description:
    //     Main.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:
    //     let result = spanda_dap::main::main();

    // Compute source for the following logic.
    let source = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: spanda-dap <file.sd>");
        std::process::exit(1);
    });
    let text = std::fs::read_to_string(&source).unwrap_or_else(|e| {
        eprintln!("Error reading {source}: {e}");
        std::process::exit(1);
    });
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut stdout = io::stdout();

    // Handle the error returned from as str.
    if let Err(e) = serve(&text, Some(source.as_str()), &mut reader, &mut stdout) {
        eprintln!("DAP server error: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resumable_machine_across_continue_requests() {
        // Description:
        //     Resumable machine across continue requests.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_dap::main::resumable_machine_across_continue_requests();

        let source = r#"
robot R {
  actuator wheels: DifferentialDrive;
  behavior run() {
    let x = 1;
    wheels.stop();
  }
}
"#;
        let breakpoints = HashSet::new();
        let s1 = with_machine(source, None, &breakpoints, |m| {
            m.run_until_pause(DebugStepKind::StepOver)
        })
        .expect("first step");
        assert!(!s1.pauses.is_empty());
        let s2 = with_machine(source, None, &breakpoints, |m| {
            m.run_until_pause(DebugStepKind::StepOver)
        })
        .expect("second step");
        assert!(!s2.pauses.is_empty());
    }
}
