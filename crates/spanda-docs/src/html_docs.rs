//! HTML documentation renderer for Spanda program API docs.

/// Wrap markdown body in a minimal HTML document shell.
pub fn markdown_to_html(title: &str, markdown_body: &str) -> String {
    // Description:
    //     Markdown to html.
    //
    // Inputs:
    //     itle: &str
    //         Caller-supplied itle.
    //     arkdown_body: &str
    //         Caller-supplied arkdown body.
    //
    // Outputs:
    //     result: String
    //         Return value from `markdown_to_html`.
    //
    // Example:

    //     let result = spanda_docs::html_docs::markdown_to_html(itle, arkdown_body);

    let body = render_markdown_fragment(markdown_body);
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{title} — Spanda API</title>
  <style>
    body {{ font-family: system-ui, sans-serif; max-width: 52rem; margin: 2rem auto; padding: 0 1rem; line-height: 1.5; color: #1a1a1a; }}
    h1, h2, h3 {{ line-height: 1.2; }}
    code {{ background: #f4f4f5; padding: 0.1em 0.35em; border-radius: 4px; font-size: 0.92em; }}
    pre {{ background: #f4f4f5; padding: 1rem; overflow-x: auto; border-radius: 6px; }}
    .doc-comment {{ color: #3f3f46; margin: 0.5rem 0 1rem; padding-left: 0.75rem; border-left: 3px solid #d4d4d8; }}
    ul {{ padding-left: 1.25rem; }}
  </style>
</head>
<body>
{body}
</body>
</html>
"#
    )
}

fn render_markdown_fragment(md: &str) -> String {
    // Description:

    //     Render markdown fragment.

    //

    // Inputs:

    //     d: &str

    //         Caller-supplied d.

    //

    // Outputs:

    //     result: String

    //         Return value from `render_markdown_fragment`.

    //

    // Example:

    //     let result = spanda_docs::html_docs::render_markdown_fragment(d);
    let mut out = String::new();
    let mut in_code = false;
    let mut list_open = false;

    for line in md.lines() {
        if line.starts_with("```") {
            if in_code {
                out.push_str("</pre>\n");
                in_code = false;
            } else {
                close_list(&mut out, &mut list_open);
                out.push_str("<pre><code>");
                in_code = true;
            }
            continue;
        }
        if in_code {
            out.push_str(&html_escape(line));
            out.push('\n');
            continue;
        }
        if let Some(rest) = line.strip_prefix("### ") {
            close_list(&mut out, &mut list_open);
            out.push_str(&format!("<h3>{}</h3>\n", inline_md(rest)));
        } else if let Some(rest) = line.strip_prefix("## ") {
            close_list(&mut out, &mut list_open);
            out.push_str(&format!("<h2>{}</h2>\n", inline_md(rest)));
        } else if let Some(rest) = line.strip_prefix("# ") {
            close_list(&mut out, &mut list_open);
            out.push_str(&format!("<h1>{}</h1>\n", inline_md(rest)));
        } else if let Some(rest) = line.strip_prefix("- ") {
            if !list_open {
                out.push_str("<ul>\n");
                list_open = true;
            }
            out.push_str(&format!("<li>{}</li>\n", inline_md(rest)));
        } else if line.is_empty() {
            close_list(&mut out, &mut list_open);
        } else {
            close_list(&mut out, &mut list_open);
            out.push_str(&format!(
                "<p class=\"doc-comment\">{}</p>\n",
                inline_md(line)
            ));
        }
    }
    if in_code {
        out.push_str("</code></pre>\n");
    }
    close_list(&mut out, &mut list_open);
    out
}

fn close_list(out: &mut String, list_open: &mut bool) {
    // Description:
    //     Close list.
    //
    // Inputs:
    //     o: &mut String
    //         Caller-supplied o.
    //     list_open: &mut bool
    //         Caller-supplied list open.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_docs::html_docs::close_list(o, list_open);

    if *list_open {
        out.push_str("</ul>\n");
        *list_open = false;
    }
}

#[allow(clippy::while_let_on_iterator)]
fn inline_md(text: &str) -> String {
    // Description:
    //     Inline md.
    //
    // Inputs:
    //     ex: &str
    //         Caller-supplied ex.
    //
    // Outputs:
    //     result: String
    //         Return value from `inline_md`.
    //
    // Example:

    //     let result = spanda_docs::html_docs::inline_md(ex);

    let mut out = String::new();
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '`' {
            out.push_str("<code>");
            while let Some(c) = chars.next() {
                if c == '`' {
                    out.push_str("</code>");
                    break;
                }
                out.push_str(&html_escape(&c.to_string()));
            }
        } else if ch == '*' {
            if chars.peek() == Some(&'*') {
                chars.next();
                out.push_str("<strong>");
                while let Some(c) = chars.next() {
                    if c == '*' && chars.peek() == Some(&'*') {
                        chars.next();
                        out.push_str("</strong>");
                        break;
                    }
                    out.push_str(&html_escape(&c.to_string()));
                }
            } else {
                out.push_str("<em>");
                while let Some(c) = chars.next() {
                    if c == '*' {
                        out.push_str("</em>");
                        break;
                    }
                    out.push_str(&html_escape(&c.to_string()));
                }
            }
        } else {
            out.push_str(&html_escape(&ch.to_string()));
        }
    }
    out
}

fn html_escape(s: &str) -> String {
    // Description:
    //     Html escape.
    //
    // Inputs:
    //     s: &str
    //         Caller-supplied s.
    //
    // Outputs:
    //     result: String
    //         Return value from `html_escape`.
    //
    // Example:

    //     let result = spanda_docs::html_docs::html_escape(s);

    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_wraps_title_and_headings() {
        // Description:
        //     Html wraps title and headings.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_docs::html_docs::html_wraps_title_and_headings();

        let html = markdown_to_html("nav", "# Module `nav`\n\n## Functions");
        assert!(html.contains("<title>nav — Spanda API</title>"));
        assert!(html.contains("<h1>"));
        assert!(html.contains("<code>nav</code>"));
    }
}
