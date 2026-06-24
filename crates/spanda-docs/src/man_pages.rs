//! Man-page lookup and optional roff generation for Spanda CLI commands.

use crate::language_reference::generate_cli_man_pages;

/// Look up a man page by command name (`run`, `spanda-run`, or `spanda run`).
pub fn lookup_man_page(query: &str) -> Option<(String, String)> {
    // Description:
    //     Lookup man page.
    //
    // Inputs:
    //     query: &str
    //         Caller-supplied query.
    //
    // Outputs:
    //     result: Option<(String, String)>
    //         Return value from `lookup_man_page`.
    //
    // Example:

    //     let result = spanda_docs::man_pages::lookup_man_page(query);

    let normalized = normalize_man_query(query);
    for (name, body) in generate_cli_man_pages() {
        let stem = name.strip_suffix(".md").unwrap_or(&name);
        let short = stem.strip_prefix("spanda-").unwrap_or(stem);
        if normalized == stem || normalized == short || normalized == format!("spanda-{short}") {
            return Some((name, body));
        }
    }
    None
}

/// List available man page names (without `.md` suffix).
pub fn list_man_pages() -> Vec<String> {
    // Description:
    //     List man pages.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: Vec<String>
    //         Return value from `list_man_pages`.
    //
    // Example:

    //     let result = spanda_docs::man_pages::list_man_pages();

    generate_cli_man_pages()
        .into_iter()
        .map(|(name, _)| name.strip_suffix(".md").unwrap_or(&name).to_string())
        .collect()
}

/// Convert man-page markdown to minimal roff for `man(1)` viewers.
pub fn markdown_man_to_roff(markdown: &str, page_name: &str) -> String {
    // Description:
    //     Markdown man to roff.
    //
    // Inputs:
    //     arkdown: &str
    //         Caller-supplied arkdown.
    //     page_name: &str
    //         Caller-supplied page name.
    //
    // Outputs:
    //     result: String
    //         Return value from `markdown_man_to_roff`.
    //
    // Example:

    //     let result = spanda_docs::man_pages::markdown_man_to_roff(arkdown, page_name);

    let section = "1";
    let mut out = String::new();
    out.push_str(&format!(
        ".TH \"{}\" \"{}\" \"Spanda\" \"Spanda CLI\"\n",
        page_name.to_uppercase(),
        section
    ));
    let mut in_code = false;
    for line in markdown.lines() {
        if line.starts_with("```") {
            in_code = !in_code;
            continue;
        }
        if in_code {
            out.push_str(".nf\n");
            out.push_str(&roff_escape(line));
            out.push_str("\n.fi\n");
            continue;
        }
        if let Some(rest) = line.strip_prefix("## ") {
            out.push_str(&format!(".SH {}\n", rest.to_uppercase()));
        } else if let Some(rest) = line.strip_prefix("# ") {
            out.push_str(&format!(".SH {}\n", rest.to_uppercase()));
        } else if let Some(rest) = line.strip_prefix("- ") {
            out.push_str(".IP \\(bu 2\n");
            out.push_str(&roff_escape(rest));
            out.push('\n');
        } else if !line.is_empty() {
            out.push_str(&roff_escape(line));
            out.push_str("\n.PP\n");
        }
    }
    out
}

fn normalize_man_query(query: &str) -> String {
    // Description:
    //     Normalize man query.
    //
    // Inputs:
    //     query: &str
    //         Caller-supplied query.
    //
    // Outputs:
    //     result: String
    //         Return value from `normalize_man_query`.
    //
    // Example:

    //     let result = spanda_docs::man_pages::normalize_man_query(query);

    let q = query
        .trim()
        .trim_start_matches("spanda ")
        .trim_start_matches("spanda-");
    if q.is_empty() || q == "spanda" {
        "spanda".into()
    } else {
        format!("spanda-{q}")
    }
}

fn roff_escape(s: &str) -> String {
    // Description:
    //     Roff escape.
    //
    // Inputs:
    //     s: &str
    //         Caller-supplied s.
    //
    // Outputs:
    //     result: String
    //         Return value from `roff_escape`.
    //
    // Example:

    //     let result = spanda_docs::man_pages::roff_escape(s);

    s.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language_reference::CLI_COMMAND_NAMES;

    #[test]
    fn registered_commands_match_man_pages() {
        // Description:
        //     Registered commands match man pages.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_docs::man_pages::registered_commands_match_man_pages();

        for cmd in CLI_COMMAND_NAMES {
            let key = cmd.strip_prefix("spanda-").unwrap_or(cmd);
            assert!(lookup_man_page(key).is_some(), "missing man page for {cmd}");
        }
    }

    #[test]
    fn lookup_verify_man_page() {
        // Description:
        //     Lookup verify man page.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_docs::man_pages::lookup_verify_man_page();

        let (name, body) = lookup_man_page("verify").expect("verify man page");
        assert_eq!(name, "spanda-verify.md");
        assert!(body.contains("## SYNOPSIS"));
        assert!(body.contains("## EXIT STATUS"));
    }

    #[test]
    fn roff_contains_th_header() {
        // Description:
        //     Roff contains th header.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_docs::man_pages::roff_contains_th_header();

        let (_, md) = lookup_man_page("check").unwrap();
        let roff = markdown_man_to_roff(&md, "spanda-check");
        assert!(roff.contains(".TH"));
        assert!(roff.contains(".SH SYNOPSIS"));
    }
}
