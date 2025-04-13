// src/html_renderer/text_processor.rs

// Extract text content from HTML nodes
pub fn get_text_content(nodes: &[html_parser::Node]) -> String {
    let mut text = String::new();
    for node in nodes {
        match node {
            html_parser::Node::Text(t) => {
                // Check if this is likely a JavaScript or CSS snippet
                let trimmed = t.trim();
                if trimmed.len() > 100 && 
                   (trimmed.contains("function") || 
                    trimmed.contains("var ") || 
                    trimmed.contains("const ") || 
                    trimmed.contains("let ") ||
                    trimmed.contains("{") && trimmed.contains("}") ||
                    trimmed.contains(";") && trimmed.contains("(") && trimmed.contains(")")) {
                    // This looks like code - truncate it
                    text.push_str("[code content removed]");
                } else if trimmed.len() > 500 {
                    // Very long text, probably not meant for display
                    text.push_str(&format!("{}... [text truncated]", &trimmed[0..100]));
                } else {
                    text.push_str(trimmed);
                }
            }
            html_parser::Node::Element(element) => {
                let tag_name = element.name.to_lowercase();
                if tag_name != "script" && tag_name != "style" {
                    text.push_str(&get_text_content(&element.children));
                }
            }
            _ => {}
        }
    }
    text.trim().to_string()
}

// Get attribute safely - using the original implementation approach
pub fn get_attribute(element: &html_parser::Element, name: &str, default: &str) -> String {
    match element.attributes.get(name) {
        Some(value) => value.clone().unwrap_or_else(|| default.to_string()),
        None => default.to_string(),
    }
}
