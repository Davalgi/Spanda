//! Spanda documentation generators — program API docs and language reference.
//!
mod builtin_methods;
pub mod language_reference;
mod program_docs;

pub use language_reference::{generate_cli_man_pages, generate_language_reference};
pub use program_docs::generate_markdown;
