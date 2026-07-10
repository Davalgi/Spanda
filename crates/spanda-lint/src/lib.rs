//! lint support for Spanda.
//!
use serde::{Deserialize, Serialize};
use spanda_ast::foundations::TaskDecl;
use spanda_ast::nodes::*;
use spanda_error::SpandaError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LintSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LintIssue {
    pub rule: String,
    pub message: String,
    pub line: u32,
    pub column: u32,
    pub severity: LintSeverity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LintReport {
    pub issues: Vec<LintIssue>,
}

impl LintReport {
    pub fn has_errors(&self) -> bool {
        // Description:
        //     Has errors.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: bool
        //         Return value from `has_errors`.
        //
        // Example:
        //     let result = spanda_lint::has_errors(&self);

        // Call issues on the current instance.
        self.issues
            .iter()
            .any(|i| i.severity == LintSeverity::Error)
    }
}

pub fn lint(source: &str) -> Result<LintReport, SpandaError> {
    // Description:
    //     Lint.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //
    // Outputs:
    //     result: Result<LintReport, SpandaError>
    //         Return value from `lint`.
    //
    // Example:
    //     let result = spanda_lint::lint(source);

    // Tokenize the source before parsing.
    let tokens = spanda_lexer::tokenize(source)?;
    let program = spanda_parser::parse(tokens)?;
    Ok(lint_program(source, &program))
}

fn lint_concurrency(program: &Program, issues: &mut Vec<LintIssue>) {
    // Description:
    //     Lint concurrency.
    //
    // Inputs:
    //     progra: &Program
    //         Caller-supplied progra.
    //     issues: &mut Vec<LintIssue>
    //         Caller-supplied issues.
    //
    // Outputs:
    //     None.
    //
    // Example:
    //     let result = spanda_lint::lint_concurrency(progra, issues);

    // Extract robot declarations from the parsed program.
    let Program::Program { robots, .. } = program;

    // Handle each robot declared in the program.
    for robot in robots {
        let RobotDecl::RobotDecl {
            behaviors, tasks, ..
        } = robot;

        // Process each behavior.
        for behavior in behaviors {
            let BehaviorDecl::BehaviorDecl { body, .. } = behavior;
            lint_stmt_channel_flow(body, issues);
        }

        // Process each task.
        for task in tasks {
            let TaskDecl::TaskDecl { body, .. } = task;
            lint_stmt_channel_flow(body, issues);
        }
    }
}

fn lint_stmt_channel_flow(stmts: &[Stmt], issues: &mut Vec<LintIssue>) {
    // Description:
    //     Lint stmt channel flow.
    //
    // Inputs:
    //     stmts: &[Stmt]
    //         Caller-supplied stmts.
    //     issues: &mut Vec<LintIssue>
    //         Caller-supplied issues.
    //
    // Outputs:
    //     None.
    //
    // Example:
    //     let result = spanda_lint::lint_stmt_channel_flow(stmts, issues);

    // Create mutable channels for accumulating results.
    let mut channels: std::collections::HashMap<String, (bool, bool, u32, u32)> =
        std::collections::HashMap::new();
    collect_channel_flow(stmts, &mut channels);

    // Iterate over channels with destructured elements.
    for (name, (sent, recv, line, column)) in channels {
        // Take this path when recv && !sent.
        if recv && !sent {
            issues.push(LintIssue {
                rule: "channel-recv-without-send".into(),
                message: format!(
                    "Channel '{name}' may be received from without a matching send in this scope"
                ),
                line,
                column,
                severity: LintSeverity::Warning,
            });
        }

        // Take this path when sent && !recv.
        if sent && !recv {
            issues.push(LintIssue {
                rule: "channel-send-without-recv".into(),
                message: format!(
                    "Channel '{name}' is sent to but never received from in this scope"
                ),
                line,
                column,
                severity: LintSeverity::Warning,
            });
        }
    }
}

#[allow(clippy::collapsible_match)]
fn collect_channel_flow(
    stmts: &[Stmt],
    channels: &mut std::collections::HashMap<String, (bool, bool, u32, u32)>,
) {
    // Description:
    //     Collect channel flow.
    //
    // Inputs:
    //     stmts: &[Stmt]
    //         Caller-supplied stmts.
    //     channels: &mut std::collections::HashMap<String, (bool, bool, u32, u32)>
    //         Caller-supplied channels.
    //
    // Outputs:
    //     None.
    //
    // Example:
    //     let result = spanda_lint::collect_channel_flow(stmts, channels);

    // Execute each statement in sequence.
    for stmt in stmts {
        // Match on stmt and handle each case.
        match stmt {
            Stmt::VarDecl {
                name, init, span, ..
            } => {
                #[allow(clippy::collapsible_if)]
                // Emit output when init provides a value.
                if let Some(value) = init {
                    // Keep entries that match the expected pattern.
                    if matches!(value, Expr::CallExpr { callee, .. }

                        // Keep entries that match the expected pattern.
                        if matches!(callee.as_ref(), Expr::IdentExpr { name: n, .. } if n == "channel"))
                    {
                        channels.entry(name.clone()).or_insert((
                            false,
                            false,
                            span.start.line,
                            span.start.column,
                        ));
                    }
                }
            }
            Stmt::ExprStmt { expr, .. }
            | Stmt::ReturnStmt {
                value: Some(expr), ..
            } => {
                mark_channel_usage(expr, channels);
            }
            Stmt::IfStmt {
                then_branch,
                else_branch,
                ..
            } => {
                collect_channel_flow(then_branch, channels);

                // Emit output when else branch provides a else branch.
                if let Some(else_branch) = else_branch {
                    collect_channel_flow(else_branch, channels);
                }
            }
            Stmt::LoopStmt { body, .. } => collect_channel_flow(body, channels),
            Stmt::ParallelStmt { body, .. } => collect_channel_flow(body, channels),
            Stmt::SelectStmt { arms, .. } => {
                // Process each arm.
                for arm in arms {
                    // Match on channel and handle each case.
                    match &arm.channel {
                        Expr::IdentExpr { name, span } => {
                            let entry = channels.entry(name.clone()).or_insert((
                                false,
                                false,
                                span.start.line,
                                span.start.column,
                            ));
                            entry.1 = true;
                        }
                        Expr::CallExpr { callee, args, .. } => {
                            // Take this path when let Expr::IdentExpr { name: fn name, .. } = callee.as ref().
                            if let Expr::IdentExpr { name: fn_name, .. } = callee.as_ref() {
                                // Take the branch when fn name equals "recv".
                                if fn_name == "recv" {
                                    // Take this path when let Some(Expr::IdentExpr { name, span }) = args.first().
                                    if let Some(Expr::IdentExpr { name, span }) = args.first() {
                                        let entry = channels.entry(name.clone()).or_insert((
                                            false,
                                            false,
                                            span.start.line,
                                            span.start.column,
                                        ));
                                        entry.1 = true;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    collect_channel_flow(&arm.body, channels);
                }
            }
            _ => {}
        }
    }
}

fn mark_channel_usage(
    expr: &Expr,
    channels: &mut std::collections::HashMap<String, (bool, bool, u32, u32)>,
) {
    // Description:
    //     Mark channel usage.
    //
    // Inputs:
    //     expr: &Expr
    //         Caller-supplied expr.
    //     channels: &mut std::collections::HashMap<String, (bool, bool, u32, u32)>
    //         Caller-supplied channels.
    //
    // Outputs:
    //     None.
    //
    // Example:
    //     let result = spanda_lint::mark_channel_usage(expr, channels);

    // Match on expr and handle each case.
    match expr {
        Expr::CallExpr { callee, args, .. } => {
            // Take this path when let Expr::IdentExpr { name: fn name, .. } = callee.as ref().
            if let Expr::IdentExpr { name: fn_name, .. } = callee.as_ref() {
                // Take the branch when fn name equals "send" || fn name == "recv".
                if fn_name == "send" || fn_name == "recv" {
                    // Take this path when let Some(Expr::IdentExpr { name, span }) = args.first().
                    if let Some(Expr::IdentExpr { name, span }) = args.first() {
                        let entry = channels.entry(name.clone()).or_insert((
                            false,
                            false,
                            span.start.line,
                            span.start.column,
                        ));

                        // Take the branch when fn name equals "send".
                        if fn_name == "send" {
                            entry.0 = true;
                        } else {
                            entry.1 = true;
                        }
                    }
                }
            }

            // Apply each command-line argument.
            for arg in args {
                mark_channel_usage(arg, channels);
            }
        }
        Expr::BinaryExpr { left, right, .. } => {
            mark_channel_usage(left, channels);
            mark_channel_usage(right, channels);
        }
        Expr::UnaryExpr { operand, .. } => mark_channel_usage(operand, channels),
        Expr::MemberExpr { object, .. } => mark_channel_usage(object, channels),
        Expr::SpawnExpr { callee, args, .. } => {
            mark_channel_usage(callee, channels);

            // Apply each command-line argument.
            for arg in args {
                mark_channel_usage(arg, channels);
            }
        }
        _ => {}
    }
}

fn lint_program(source: &str, program: &Program) -> LintReport {
    // Description:
    //     Lint program.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //     progra: &Program
    //         Caller-supplied progra.
    //
    // Outputs:
    //     result: LintReport
    //         Return value from `lint_program`.
    //
    // Example:
    //     let result = spanda_lint::lint_program(source, progra);

    // Create mutable issues for accumulating results.
    let mut issues = Vec::new();
    lint_source_style(source, &mut issues);
    lint_program_structure(program, &mut issues);
    lint_imports(source, program, &mut issues);
    lint_concurrency(program, &mut issues);
    lint_library_shaped_decls(program, &mut issues);
    LintReport { issues }
}

fn lint_library_shaped_decls(program: &Program, issues: &mut Vec<LintIssue>) {
    // Warn on thin policy decls that are migration candidates (see language-surface-inventory).
    //
    // Parameters:
    // - `program` — parsed program
    // - `issues` — lint issue accumulator
    //
    // Returns:
    // None.
    //
    // Options:
    // None.
    //
    // Example:
    // lint_library_shaped_decls(&program, &mut issues);

    let Program::Program {
        homeostasis_policies,
        attention_policies,
        ..
    } = program;

    // Flag each legacy homeostasis_policy as library-shaped surface.
    for policy in homeostasis_policies {
        let spanda_ast::assurance_decl::HomeostasisPolicyDecl::HomeostasisPolicyDecl {
            name,
            legacy_syntax,
            span,
            ..
        } = policy;

        // Skip preferred `@policy(kind: "homeostasis")` forms.
        if !legacy_syntax {
            continue;
        }
        issues.push(LintIssue {
            rule: "library-shaped-decl".into(),
            message: format!(
                "`homeostasis_policy {name}` is library-shaped syntax — prefer \
                 `@policy(kind: \"homeostasis\")` (see docs/language-surface-inventory.md)"
            ),
            line: span.start.line,
            column: span.start.column,
            severity: LintSeverity::Warning,
        });
    }

    // Flag each legacy attention_policy as library-shaped surface.
    for policy in attention_policies {
        let spanda_ast::assurance_decl::AttentionPolicyDecl::AttentionPolicyDecl {
            name,
            legacy_syntax,
            span,
            ..
        } = policy;

        // Skip preferred `@policy(kind: "attention")` forms.
        if !legacy_syntax {
            continue;
        }
        issues.push(LintIssue {
            rule: "library-shaped-decl".into(),
            message: format!(
                "`attention_policy {name}` is library-shaped syntax — prefer \
                 `@policy(kind: \"attention\")` (see docs/language-surface-inventory.md)"
            ),
            line: span.start.line,
            column: span.start.column,
            severity: LintSeverity::Warning,
        });
    }
}

fn lint_source_style(source: &str, issues: &mut Vec<LintIssue>) {
    // Description:
    //     Lint source style.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //     issues: &mut Vec<LintIssue>
    //         Caller-supplied issues.
    //
    // Outputs:
    //     None.
    //
    // Example:
    //     let result = spanda_lint::lint_source_style(source, issues);

    // Iterate over enumerate with destructured elements.
    for (idx, line) in source.lines().enumerate() {
        let line_no = idx as u32 + 1;

        // Take this path when line.ends with(' ') || line.ends with('\t').
        if line.ends_with(' ') || line.ends_with('\t') {
            issues.push(LintIssue {
                rule: "trailing-whitespace".into(),
                message: "Line has trailing whitespace".into(),
                line: line_no,
                column: line.len() as u32,
                severity: LintSeverity::Warning,
            });
        }

        // Take this path when line.len() > 120.
        if line.len() > 120 {
            issues.push(LintIssue {
                rule: "line-length".into(),
                message: format!("Line exceeds 120 columns ({} chars)", line.len()),
                line: line_no,
                column: 121,
                severity: LintSeverity::Warning,
            });
        }
    }
}

fn lint_program_structure(program: &Program, issues: &mut Vec<LintIssue>) {
    // Description:
    //     Lint program structure.
    //
    // Inputs:
    //     progra: &Program
    //         Caller-supplied progra.
    //     issues: &mut Vec<LintIssue>
    //         Caller-supplied issues.
    //
    // Outputs:
    //     None.
    //
    // Example:
    //     let result = spanda_lint::lint_program_structure(progra, issues);

    // Destructure the program into its top-level sections.
    let Program::Program {
        module_name,
        tests,
        robots,
        ..
    } = program;

    // Take this path when module name.is none().
    if module_name.is_none() {
        issues.push(LintIssue {
            rule: "missing-module".into(),
            message: "Program has no `module` declaration".into(),
            line: 1,
            column: 1,
            severity: LintSeverity::Warning,
        });
    }

    // Run each test block in program order.
    for test in tests {
        // Skip further work when body is empty.
        if test.body.is_empty() {
            issues.push(LintIssue {
                rule: "empty-test".into(),
                message: format!("Test \"{}\" has an empty body", test.name),
                line: test.span.start.line,
                column: test.span.start.column,
                severity: LintSeverity::Warning,
            });
        }
    }

    // Handle each robot declared in the program.
    for robot in robots {
        let RobotDecl::RobotDecl {
            behaviors, verify, ..
        } = robot;

        // Prefer assert { } for runtime assertion blocks (vocabulary clarity).
        if let Some(spanda_ast::foundations::VerifyDecl::VerifyDecl {
            assert_alias, span, ..
        }) = verify
        {
            if !assert_alias {
                issues.push(LintIssue {
                    rule: "verify-block-alias".into(),
                    message: "`verify { }` is a runtime assertion block (not formal verification \
                         or `spanda verify`). Prefer `assert { }` — see docs/verification-vocabulary.md"
                        .into(),
                    line: span.start.line,
                    column: span.start.column,
                    severity: LintSeverity::Warning,
                });
            }
        }

        // Process each behavior.
        for behavior in behaviors {
            let BehaviorDecl::BehaviorDecl {
                name, body, span, ..
            } = behavior;

            // Skip further work when body is empty.
            if body.is_empty() {
                issues.push(LintIssue {
                    rule: "empty-behavior".into(),
                    message: format!("Behavior `{name}` has an empty body"),
                    line: span.start.line,
                    column: span.start.column,
                    severity: LintSeverity::Warning,
                });
            }
        }
    }
}

fn lint_imports(source: &str, program: &Program, issues: &mut Vec<LintIssue>) {
    // Description:
    //     Lint imports.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //     progra: &Program
    //         Caller-supplied progra.
    //     issues: &mut Vec<LintIssue>
    //         Caller-supplied issues.
    //
    // Outputs:
    //     None.
    //
    // Example:
    //     let result = spanda_lint::lint_imports(source, progra, issues);

    // Destructure the program into its top-level sections.
    let Program::Program { imports, .. } = program;

    // Emit codegen metadata for each import.
    for import in imports {
        let ImportDecl::ImportDecl { path, span } = import;
        let needle = path.split('.').next_back().unwrap_or(path.as_str());
        let referenced = source.matches(needle).count() > 1
            || source.contains(&format!("{path}::"))
            || source.contains(&format!("from {path}"))
            || is_std_import(path);

        // Take the branch when referenced is false.
        if !referenced {
            issues.push(LintIssue {
                rule: "unused-import".into(),
                message: format!("Import `{path}` appears unused"),
                line: span.start.line,
                column: span.start.column,
                severity: LintSeverity::Warning,
            });
        }
    }
}

fn is_std_import(path: &str) -> bool {
    // Description:
    //     Is std import.
    //
    // Inputs:
    //     path: &str
    //         Caller-supplied path.
    //
    // Outputs:
    //     result: bool
    //         Return value from `is_std_import`.
    //
    // Example:
    //     let result = spanda_lint::is_std_import(path);

    // Produce ") as the result.
    path.starts_with("std.") || path.starts_with("sensors.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_missing_module_and_trailing_space() {
        // Description:
        //     Detects missing module and trailing space.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_lint::detects_missing_module_and_trailing_space();

        let source = "robot R {  \n  actuator wheels: DifferentialDrive;\n}\n";
        let report = lint(source).expect("lint should parse");
        assert!(report.issues.iter().any(|i| i.rule == "missing-module"));
        assert!(report
            .issues
            .iter()
            .any(|i| i.rule == "trailing-whitespace"));
    }

    #[test]
    fn warns_on_verify_block_preferring_assert_alias() {
        // Runtime assertion blocks named verify { } get a vocabulary lint.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // None.
        //
        // Options:
        // None.
        //
        // Example:
        // warns_on_verify_block_preferring_assert_alias();

        let source = r#"
module demo;
robot R {
  actuator wheels: DifferentialDrive;
  verify { robot.velocity().linear <= 1.0 m/s; }
  behavior b() { wheels.stop(); }
}
"#;
        let report = lint(source).expect("lint should parse");
        assert!(
            report.issues.iter().any(|i| i.rule == "verify-block-alias"),
            "expected verify-block-alias warning, got {:?}",
            report.issues
        );

        let preferred = r#"
module demo;
robot R {
  actuator wheels: DifferentialDrive;
  assert { robot.velocity().linear <= 1.0 m/s; }
  behavior b() { wheels.stop(); }
}
"#;
        let ok = lint(preferred).expect("lint should parse");
        assert!(
            !ok.issues.iter().any(|i| i.rule == "verify-block-alias"),
            "assert {{ }} should not warn, got {:?}",
            ok.issues
        );
    }

    #[test]
    fn warns_on_library_shaped_homeostasis_policy() {
        // Thin policy decls get a migration-path lint.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // None.
        //
        // Options:
        // None.
        //
        // Example:
        // warns_on_library_shaped_homeostasis_policy();

        let source = r#"
module demo;
homeostasis_policy KeepAlive {
  metric battery_pct;
}
robot R {
  actuator wheels: DifferentialDrive;
  behavior b() { wheels.stop(); }
}
"#;
        let report = lint(source).expect("lint should parse");
        assert!(
            report.issues.iter().any(
                |i| i.rule == "library-shaped-decl" && i.message.contains("homeostasis_policy")
            ),
            "expected library-shaped-decl for homeostasis_policy, got {:?}",
            report.issues
        );
    }

    #[test]
    fn at_policy_forms_skip_library_shaped_lint() {
        // Preferred `@policy` attribute forms should not warn.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // None.
        //
        // Options:
        // None.
        //
        // Example:
        // at_policy_forms_skip_library_shaped_lint();

        let source = r#"
module demo;
@policy(kind: "homeostasis")
KeepAlive {
  metric battery_pct;
}
@policy(kind: "attention")
Focus {
  rule suppress_low_priority;
}
robot R {
  actuator wheels: DifferentialDrive;
  behavior b() { wheels.stop(); }
}
"#;
        let report = lint(source).expect("lint should parse");
        assert!(
            !report
                .issues
                .iter()
                .any(|i| i.rule == "library-shaped-decl"),
            "preferred @policy forms should not warn, got {:?}",
            report.issues
        );
    }

    #[test]
    fn detects_empty_test_block() {
        // Description:
        //     Detects empty test block.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_lint::detects_empty_test_block();

        let source = r#"
module tests;

test "noop" {
}
"#;
        let report = lint(source).expect("lint should parse");
        assert!(report.issues.iter().any(|i| i.rule == "empty-test"));
    }
}
