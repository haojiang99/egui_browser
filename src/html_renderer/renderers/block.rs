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
    ui.add_space(2.0);
    ui.group(|ui| {
        renderer.render_html_node(ui, &element.children);
    });
    ui.add_space(2.0);
}

// Render code block
pub fn render_code(
    ui: &mut Ui, 
    element: &html_parser::Element, 
    _renderer: &HtmlRenderer
) {
    let text = get_text_content(&element.children);
    ui.add(
        TextEdit::multiline(&mut text.clone())
            .font(egui::TextStyle::Monospace)
            .desired_width(f32::INFINITY)
            .interactive(false)
    );
}
