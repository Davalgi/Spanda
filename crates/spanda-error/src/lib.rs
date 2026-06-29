//! Compiler and runtime error types shared across Spanda crates.
//!
pub use spanda_typecheck::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpandaError {
    #[error("{message} (line {line}, col {column})")]
    Lexer {
        message: String,
        line: u32,
        column: u32,
    },
    #[error("{message} (line {line}, col {column})")]
    Parse {
        message: String,
        line: u32,
        column: u32,
    },
    #[error("Type check failed")]
    TypeCheck { diagnostics: Vec<Diagnostic> },
    #[error("{message} (line {line})")]
    Runtime { message: String, line: u32 },
    #[error("Debug pause at line {line}: {reason}")]
    DebugPause { line: u32, reason: String },
}

impl SpandaError {
    pub fn diagnostics(&self) -> Vec<Diagnostic> {
        // Description:
        //     Diagnostics.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Vec<Diagnostic>
        //         Return value from `diagnostics`.
        //
        // Example:

        //     let result = spanda_error::diagnostics(&self);

        match self {
            SpandaError::Lexer {
                message,
                line,
                column,
            } => vec![Diagnostic {
                message: message.clone(),
                line: *line,
                column: *column,
            }],
            SpandaError::Parse {
                message,
                line,
                column,
            } => vec![Diagnostic {
                message: message.clone(),
                line: *line,
                column: *column,
            }],
            SpandaError::TypeCheck { diagnostics } => diagnostics.clone(),
            SpandaError::Runtime { message, line } => vec![Diagnostic {
                message: message.clone(),
                line: *line,
                column: 1,
            }],
            SpandaError::DebugPause { line, reason } => vec![Diagnostic {
                message: format!("Debug pause: {reason}"),
                line: *line,
                column: 1,
            }],
        }
    }
}

impl From<spanda_lexer::LexerError> for SpandaError {
    fn from(err: spanda_lexer::LexerError) -> Self {
        // Description:
        //     From.
        //
        // Inputs:
        //     err: spanda_lexer::LexerError
        //         Caller-supplied err.
        //
        // Outputs:
        //     result: Self
        //         Return value from `from`.
        //
        // Example:

        //     let result = spanda_error::from(err);

        Self::Lexer {
            message: err.message,
            line: err.line,
            column: err.column,
        }
    }
}

impl From<spanda_ast::RegexCompileError> for SpandaError {
    fn from(err: spanda_ast::RegexCompileError) -> Self {
        // Description:
        //     From.
        //
        // Inputs:
        //     err: spanda_ast::RegexCompileError
        //         Caller-supplied err.
        //
        // Outputs:
        //     result: Self
        //         Return value from `from`.
        //
        // Example:

        //     let result = spanda_error::from(err);

        Self::Parse {
            message: err.message,
            line: err.line,
            column: err.column,
        }
    }
}
