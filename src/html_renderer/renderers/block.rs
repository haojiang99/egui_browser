// src/html_renderer/renderers/block.rs
use egui::{TextEdit, Ui};
use crate::html_renderer::renderer::HtmlRenderer;
use crate::html_renderer::text_processor::get_text_content;

// Render block element
pub fn render_block_element(
    ui: &mut Ui, 
    element: &html_parser::Element,
    renderer: &HtmlRenderer
) {
    // Check if we need to render as a group (reduce nesting)
    // Only create a group for elements with certain attributes or multiple children
    let has_id = element.attributes.contains_key("id");
    let has_class = element.attributes.contains_key("class");
    let multiple_children = element.children.len() > 1;
    
    if has_id || has_class || multiple_children {
        // Add minimal spacing and group
        ui.add_space(1.0);
        ui.group(|ui| {
            renderer.render_html_node(ui, &element.children);
        });
        ui.add_space(1.0);
    } else {
        // Skip the group and render children directly to avoid nesting
        renderer.render_html_node(ui, &element.children);
    }
}

// Render code block
pub fn render_code(
    ui: &mut Ui, 
    element: &html_parser::Element, 
    _renderer: &HtmlRenderer
) {
    let text = get_text_content(&element.children);
    
    // Truncate very long code blocks
    let max_length = 200; // Maximum characters to display
    let display_text = if text.len() > max_length {
        format!("{}... [code truncated, {} more characters]", 
                &text[0..max_length], 
                text.len() - max_length)
    } else {
        text
    };
    
    ui.add(
        TextEdit::multiline(&mut display_text.clone())
            .font(egui::TextStyle::Monospace)
            .desired_width(f32::INFINITY)
            .interactive(false)
    );
}
