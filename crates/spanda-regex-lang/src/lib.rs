//! First-class regex compilation, validation, and runtime matching for Spanda.
//!
use spanda_ast::nodes::Span;
pub use spanda_ast::{CaptureResult, RegexCompileError, RegexPattern};
use spanda_error::SpandaError;
use std::collections::HashMap;

pub fn regex_matches(pattern: &RegexPattern, text: &str) -> Result<bool, SpandaError> {
    // Description:
    //     Regex matches.
    //
    // Inputs:
    //     pattern: &RegexPattern
    //         Caller-supplied pattern.
    //     ex: &str
    //         Caller-supplied ex.
    //
    // Outputs:
    //     result: Result<bool, SpandaError>
    //         Return value from `regex_matches`.
    //
    // Example:
    //     let result = spanda_regex_lang::regex_matches(pattern, ex);

    // Compile once and test the entire input string.
    let re = pattern.compile()?;
    Ok(re.is_match(text))
}

pub fn regex_find(pattern: &RegexPattern, text: &str) -> Result<Option<String>, SpandaError> {
    // Description:
    //     Regex find.
    //
    // Inputs:
    //     pattern: &RegexPattern
    //         Caller-supplied pattern.
    //     ex: &str
    //         Caller-supplied ex.
    //
    // Outputs:
    //     result: Result<Option<String>, SpandaError>
    //         Return value from `regex_find`.
    //
    // Example:
    //     let result = spanda_regex_lang::regex_find(pattern, ex);

    // Compile once and return the first match slice as owned text.
    let re = pattern.compile()?;
    Ok(re.find(text).map(|m| m.as_str().to_string()))
}

pub fn regex_replace(
    pattern: &RegexPattern,
    text: &str,
    replacement: &str,
) -> Result<String, SpandaError> {
    // Description:
    //     Regex replace.
    //
    // Inputs:
    //     pattern: &RegexPattern
    //         Caller-supplied pattern.
    //     ex: &str
    //         Caller-supplied ex.
    //     replacemen: &str
    //         Caller-supplied replacemen.
    //
    // Outputs:
    //     result: Result<String, SpandaError>
    //         Return value from `regex_replace`.
    //
    // Example:
    //     let result = spanda_regex_lang::regex_replace(pattern, ex, replacemen);

    // Compile once and apply global replacement.
    let re = pattern.compile()?;
    Ok(re.replace_all(text, replacement).into_owned())
}

pub fn regex_split(pattern: &RegexPattern, text: &str) -> Result<Vec<String>, SpandaError> {
    // Description:
    //     Regex split.
    //
    // Inputs:
    //     pattern: &RegexPattern
    //         Caller-supplied pattern.
    //     ex: &str
    //         Caller-supplied ex.
    //
    // Outputs:
    //     result: Result<Vec<String>, SpandaError>
    //         Return value from `regex_split`.
    //
    // Example:
    //     let result = spanda_regex_lang::regex_split(pattern, ex);

    // Compile once and split on every match boundary.
    let re = pattern.compile()?;
    Ok(re.split(text).map(str::to_string).collect())
}

pub fn regex_capture(
    pattern: &RegexPattern,
    text: &str,
) -> Result<Option<CaptureResult>, SpandaError> {
    // Description:
    //     Regex capture.
    //
    // Inputs:
    //     pattern: &RegexPattern
    //         Caller-supplied pattern.
    //     ex: &str
    //         Caller-supplied ex.
    //
    // Outputs:
    //     result: Result<Option<CaptureResult>, SpandaError>
    //         Return value from `regex_capture`.
    //
    // Example:
    //     let result = spanda_regex_lang::regex_capture(pattern, ex);

    // Compile once and extract the first match with named groups.
    let re = pattern.compile()?;
    let Some(caps) = re.captures(text) else {
        return Ok(None);
    };
    let full = caps
        .get(0)
        .map(|m| m.as_str().to_string())
        .unwrap_or_default();
    let mut groups = HashMap::new();
    for name in re.capture_names().flatten() {
        if let Some(m) = caps.name(name) {
            groups.insert(name.to_string(), m.as_str().to_string());
        }
    }
    Ok(Some(CaptureResult { full, groups }))
}

pub fn validate_regex_literal(source: &str, flags: &str, span: Span) -> Result<(), SpandaError> {
    // Description:
    //     Validate regex literal.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //     flags: &str
    //         Caller-supplied flags.
    //     span: Span
    //         Caller-supplied span.
    //
    // Outputs:
    //     result: Result<(), SpandaError>
    //         Return value from `validate_regex_literal`.
    //
    // Example:
    //     let result = spanda_regex_lang::validate_regex_literal(source, flags, span);

    // Compile through the shared helper so diagnostics stay consistent.
    let pattern = RegexPattern {
        source: source.to_string(),
        flags: flags.to_string(),
        span,
    };
    pattern.compile().map_err(SpandaError::from).map(|_| ())
}
