//! Property-style parser tests for release hardening.

use spanda_lexer::tokenize;
use spanda_parser::parse;

fn try_parse(source: &str) -> bool {
    tokenize(source)
        .ok()
        .and_then(|tokens| parse(tokens).ok())
        .is_some()
}

#[test]
fn parser_never_panics_on_random_ascii() {
    // Random ASCII inputs must not panic the parser.
    let seeds = [
        "",
        " ",
        "\n\t",
        "robot",
        "robot {}",
        "robot X {",
        "{{{{",
        "}}}}",
        "sensor lidar: Lidar;",
        "/* unterminated",
        "\"unterminated",
        "robot X { sensor a: B; }",
        &"a".repeat(10_000),
        &"{".repeat(256),
        "robot R { behavior b() { loop every 10ms { } } }",
    ];
    for source in seeds {
        let _ = std::panic::catch_unwind(|| {
            let _ = try_parse(source);
        })
        .expect("parser must not panic");
    }
}

#[test]
fn parser_accepts_minimal_robot_program() {
    // Minimal valid programs must parse.
    assert!(try_parse(
        r#"
        robot Patrol {
          sensor lidar: Lidar;
          actuator wheels: DifferentialDrive;
          behavior go() {
            loop every 100ms { }
          }
        }
        "#
    ));
}

#[test]
fn parser_rejects_obviously_invalid_programs() {
    // Broken programs must fail without panicking.
    assert!(!try_parse("not a program"));
    assert!(!try_parse("robot { }"));
}
