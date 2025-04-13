// src/ui_components.rs
use egui::{ScrollArea, TextEdit, Ui};
use crate::html_renderer::HtmlRenderer;

pub fn render_html_content(ui: &mut Ui, html_parser: &html_parser::Dom, html_renderer: &HtmlRenderer) {
    ui.separator();
    
    // Create a frame with white background for rendered HTML
    let html_frame = egui::Frame::default()
        .fill(egui::Color32::from_rgb(255, 255, 255))
        .inner_margin(egui::style::Margin::same(10.0));
        
    html_frame.show(ui, |ui| {
        ScrollArea::vertical().show(ui, |ui| {
            // Find the body tag and get filtered content for rendering
            if let Some(filtered_body) = html_renderer.find_body_element(&html_parser.children) {
                html_renderer.render_html_node(ui, &filtered_body);
            } else {
                // If no body tag is found, filter and render everything
                let filtered_content = html_renderer.filter_nodes(&html_parser.children);
                html_renderer.render_html_node(ui, &filtered_content);
            }
        });
    });
}

pub fn render_raw_html_view(ui: &mut Ui, html: &str, show_html: &mut bool) {
    ui.separator();
    
    // Add a toggle button to show/hide the raw HTML
    ui.horizontal(|ui| {
        ui.heading("Raw HTML:");
        ui.checkbox(show_html, "Show Source");
    });
    
    // Only show the raw HTML if the toggle is on
    if *show_html {
        let mut text = html.to_string();
        ui.add(TextEdit::multiline(&mut text).desired_width(f32::INFINITY).desired_rows(10));
    }
}