// src/html_renderer/text_processor.rs

// Extract text content from HTML nodes
pub fn get_text_content(nodes: &[html_parser::Node]) -> String {
    let mut text = String::new();
    for node in nodes {
        match node {
            html_parser::Node::Text(t) => {
                text.push_str(t);
            }
            html_parser::Node::Element(element) => {
                text.push_str(&get_text_content(&element.children));
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
