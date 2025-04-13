// src/ui_components.rs
use egui::{ScrollArea, TextEdit, Ui};
use crate::html_renderer::HtmlRenderer;

pub fn render_raw_html_view(ui: &mut Ui, html: &str) {
    ui.separator();
    ui.heading("Raw HTML:");
    
    // Create a collapsing region to hide/show raw HTML
    egui::collapsing_header::CollapsingHeader::new("View Source")
        .default_open(false)
        .show(ui, |ui| {
            let mut text = html.to_string();
            ui.add(TextEdit::multiline(&mut text).desired_width(f32::INFINITY));
        });
}

pub fn render_html_content(ui: &mut Ui, html_parser: &html_parser::Dom, html_renderer: &HtmlRenderer) {
    ui.separator();
    ui.heading("Rendered HTML:");
    
    ScrollArea::vertical().show(ui, |ui| {
        // Find the body tag for rendering
        let body = html_renderer.find_body_element(&html_parser.children);
        if let Some(body_children) = body {
            html_renderer.render_html_node(ui, body_children);
        } else {
            // If no body tag is found, render everything
            html_renderer.render_html_node(ui, &html_parser.children);
        }
    });
}