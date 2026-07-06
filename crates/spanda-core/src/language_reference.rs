//! Language reference shim — delegates to `spanda_docs`.
pub use spanda_docs::{
    generate_cli_man_pages, generate_language_reference, list_man_pages, lookup_man_page,
    markdown_man_to_roff, CLI_COMMAND_NAMES,
};
