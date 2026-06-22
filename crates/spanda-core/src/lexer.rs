//! Spanda lexer re-exported from `spanda-lexer`.
//!
pub use spanda_lexer::*;

use crate::error::SpandaError;

/// Tokenize Spanda source into a token stream (maps `LexerError` to `SpandaError`).
pub fn tokenize(source: &str) -> Result<Vec<Token>, SpandaError> {
    // Delegate tokenization to the extracted lexer crate.
    //
    // Parameters:
    // - `source` — full `.sd` source text
    //
    // Returns:
    // Token vector, or a lexer diagnostic as `SpandaError::Lexer`.
    //
    // Options:
    // None.
    //
    // Example:
    // let tokens = tokenize(source)?;

    spanda_lexer::tokenize(source).map_err(SpandaError::from)
}
