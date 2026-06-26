//! Minimal text-only PDF generation for executive report exports.
//!

/// Render a plain-text document as a minimal PDF 1.4 byte stream.
pub fn render_text_pdf(title: &str, body: &str) -> Vec<u8> {
    let mut lines: Vec<String> = vec![title.to_string(), String::new()];
    lines.extend(
        body.lines()
            .take(80)
            .map(|line| sanitize_pdf_text(line)),
    );
    let content = format!(
        "BT\n/F1 11 Tf\n14 TL\n50 750 Td\n{} T*\nET",
        lines
            .iter()
            .map(|line| format!("({}) Tj T*", escape_pdf_string(line)))
            .collect::<Vec<_>>()
            .join("\n")
    );
    let objects = vec![
        "1 0 obj<< /Type /Catalog /Pages 2 0 R >>endobj".to_string(),
        "2 0 obj<< /Type /Pages /Kids [3 0 R] /Count 1 >>endobj".to_string(),
        "3 0 obj<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R /Resources<< /Font<< /F1 5 0 R >> >> >>endobj".to_string(),
        format!("4 0 obj<< /Length {} >>stream\n{content}\nendstream\nendobj", content.len()),
        "5 0 obj<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>endobj".to_string(),
    ];
    let mut pdf = String::from("%PDF-1.4\n");
    let mut offsets = vec![0usize];
    for object in objects {
        offsets.push(pdf.len());
        pdf.push_str(&object);
        pdf.push('\n');
    }
    let xref_start = pdf.len();
    pdf.push_str(&format!("xref\n0 {}\n", offsets.len()));
    pdf.push_str("0000000000 65535 f \n");
    for offset in offsets.iter().skip(1) {
        pdf.push_str(&format!("{offset:010} 00000 n \n"));
    }
    pdf.push_str(&format!(
        "trailer<< /Size {} /Root 1 0 R >>\nstartxref\n{xref_start}\n%%EOF",
        offsets.len()
    ));
    pdf.into_bytes()
}

fn sanitize_pdf_text(line: &str) -> String {
    line.chars()
        .filter(|ch| !ch.is_control() || *ch == '\t')
        .take(100)
        .collect()
}

fn escape_pdf_string(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pdf_has_header_and_eof() {
        let bytes = render_text_pdf("Spanda Report", "line one\nline two");
        let text = String::from_utf8_lossy(&bytes);
        assert!(text.starts_with("%PDF-1.4"));
        assert!(text.ends_with("%%EOF"));
    }
}
