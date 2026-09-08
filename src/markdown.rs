pub struct ExtractedBlock {
    pub content: String,
    pub is_mermaid: bool,
}

/// Extracts mermaid blocks or passes through full markdown with mermaid blocks replaced by rendered ASCII.
pub fn extract_mermaid_blocks(markdown: &str) -> Vec<String> {
    let mut diagrams = Vec::new();
    let mut in_mermaid_block = false;
    let mut current_block = Vec::new();

    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```mermaid") {
            in_mermaid_block = true;
            current_block.clear();
        } else if in_mermaid_block && trimmed == "```" {
            in_mermaid_block = false;
            let diagram_text = current_block.join("\n");
            if !diagram_text.trim().is_empty() {
                diagrams.push(diagram_text);
            }
            current_block.clear();
        } else if in_mermaid_block {
            current_block.push(line);
        }
    }

    diagrams
}

/// Replaces all ```mermaid ... ``` blocks in a Markdown file with their rendered ASCII diagram.
pub fn transform_markdown<F>(markdown: &str, mut renderer: F) -> String
where
    F: FnMut(&str) -> Result<String, String>,
{
    let mut output = Vec::new();
    let mut in_mermaid = false;
    let mut current_block = Vec::new();

    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```mermaid") {
            in_mermaid = true;
            current_block.clear();
        } else if in_mermaid && trimmed == "```" {
            in_mermaid = false;
            let diagram_code = current_block.join("\n");
            match renderer(&diagram_code) {
                Ok(rendered) => {
                    output.push("```text".to_string());
                    output.push(rendered.trim_end().to_string());
                    output.push("```".to_string());
                }
                Err(e) => {
                    // Fallback to original block with error
                    output.push(format!("<!-- Mermaid render error: {} -->", e));
                    output.push("```mermaid".to_string());
                    output.push(diagram_code);
                    output.push("```".to_string());
                }
            }
            current_block.clear();
        } else if in_mermaid {
            current_block.push(line);
        } else {
            output.push(line.to_string());
        }
    }

    output.join("\n")
}
