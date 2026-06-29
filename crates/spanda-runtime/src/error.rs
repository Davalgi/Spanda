//! Runtime execution errors surfaced by the Spanda interpreter.
//!

/// Interpreter failure with source line attribution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeError {
    pub message: String,
    pub line: u32,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (line {})", self.message, self.line)
    }
}

impl RuntimeError {
    pub fn new(message: impl Into<String>, line: u32) -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     essage: impl Into<String>
        //         Caller-supplied essage.
        //     line: u32
        //         Caller-supplied line.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:

        //     let value = spanda_runtime::error::new(essage, line);

        Self {
            message: message.into(),
            line,
        }
    }
}

impl From<RuntimeError> for spanda_error::SpandaError {
    fn from(err: RuntimeError) -> Self {
        // Lift runtime failures into shared SpandaError diagnostics.
        Self::Runtime {
            message: err.message,
            line: err.line,
        }
    }
}
