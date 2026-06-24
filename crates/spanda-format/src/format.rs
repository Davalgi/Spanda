//! Source formatter for Spanda programs.
//!
//! Applies AST-aware pretty printing when parsing succeeds; falls back to
//! whitespace normalization (trim trailing spaces, ensure final newline).

use crate::pretty::pretty_print_program;
use spanda_error::SpandaError;

pub fn format_source(source: &str) -> String {
    // Description:
    //     Format source.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //
    // Outputs:
    //     result: String
    //         Return value from `format_source`.
    //
    // Example:
    //     let result = spanda_format::format::format_source(source);

    // assert!(out.contains("export fn f(x: Int) -> Int"));
    match format_ast(source) {
        Ok(formatted) => formatted,
        Err(_) => normalize_whitespace(source),
    }
}

pub fn format_ast(source: &str) -> Result<String, SpandaError> {
    // Description:
    //     Format ast.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //
    // Outputs:
    //     result: Result<String, SpandaError>
    //         Return value from `format_ast`.
    //
    // Example:
    //     let result = spanda_format::format::format_ast(source);

    // assert!(out.contains("export fn f()"));
    let tokens = spanda_lexer::tokenize(source)?;
    let program = spanda_parser::parse(tokens)?;
    Ok(pretty_print_program(source, &program))
}

fn normalize_whitespace(source: &str) -> String {
    // Description:
    //     Normalize whitespace.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //
    // Outputs:
    //     result: String
    //         Return value from `normalize_whitespace`.
    //
    // Example:
    //     let result = spanda_format::format::normalize_whitespace(source);

    // Start the generated output buffer.
    let mut out = String::new();

    // Handle each input line.
    for line in source.lines() {
        out.push_str(line.trim_end());
        out.push('\n');
    }

    // Repeat while out.ends with("\n\n").
    while out.ends_with("\n\n") {
        out.pop();
    }

    // Take the branch when ends with is false.
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_trailing_whitespace_and_adds_final_newline() {
        // Description:
        //     Trims trailing whitespace and adds final newline.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_format::format::trims_trailing_whitespace_and_adds_final_newline();

        let input = "robot R {  \n  actuator wheels: DifferentialDrive; \n}\n\n";
        let formatted = format_source(input);
        assert!(formatted.ends_with('\n'));
        assert!(!formatted.contains("  \n"));
    }

    #[test]
    fn ast_format_normalizes_module_function() {
        // Description:
        //     Ast format normalizes module function.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_format::format::ast_format_normalizes_module_function();

        let input = "module m;\nexport fn f(x:Int)->Int{return x;}\n";
        let formatted = format_source(input);
        assert!(formatted.contains("export fn f(x: Int) -> Int"));
    }
}
