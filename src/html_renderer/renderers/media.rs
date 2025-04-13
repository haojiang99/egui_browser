// src/html_renderer/renderers/media.rs
use egui::Ui;
use crate::html_renderer::text_processor::get_attribute;

// Render image
pub fn render_image(
    ui: &mut Ui, 
    element: &html_parser::Element
) {
    let alt = get_attribute(element, "alt", "");
    if !alt.is_empty() {
        ui.label(format!("[Image: {}]", alt));
    } else {
        ui.label("[Image]");
    }
}
