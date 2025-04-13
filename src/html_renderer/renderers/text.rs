// src/html_renderer/renderers/text.rs
use crate::style::ElementStyle;
use egui::{RichText, Ui};
use crate::html_renderer::renderer::HtmlRenderer;
use crate::html_renderer::style_handler::{apply_style, get_link_style};
use crate::html_renderer::text_processor::get_text_content;

// Render heading (h1-h6)
pub fn render_heading(
    ui: &mut Ui, 
    element: &html_parser::Element, 
    style: Option<&ElementStyle>,
    _renderer: &HtmlRenderer
) {
    let text = get_text_content(&element.children);
    let rich_text = apply_style(&text, style);
    
    ui.add_space(4.0);
    ui.heading(rich_text);
    ui.add_space(4.0);
}

// Render paragraph
pub fn render_paragraph(
    ui: &mut Ui, 
    element: &html_parser::Element, 
    style: Option<&ElementStyle>,
    _renderer: &HtmlRenderer
) {
    let text = get_text_content(&element.children);
    let rich_text = apply_style(&text, style);
    
    ui.label(rich_text);
    ui.add_space(4.0);
}

// Render link
pub fn render_link(
    ui: &mut Ui, 
    element: &html_parser::Element,
    _renderer: &HtmlRenderer
) {
    let text = get_text_content(&element.children);
    
    if element.attributes.contains_key("href") {
        let rich_text = get_link_style(&text);
        
        if ui.link(rich_text).clicked() {
            println!("Link clicked");
        }
    } else {
        ui.label(text);
    }
}

// Render text formatting (strong, em, etc.)
pub fn render_text_formatting(
    ui: &mut Ui, 
    element: &html_parser::Element,
    tag: &str,
    _renderer: &HtmlRenderer
) {
    let text = get_text_content(&element.children);
    
    match tag {
        "strong" | "b" => {
            ui.label(RichText::new(text).strong());
        }
        "em" | "i" => {
            ui.label(RichText::new(text).italics());
        }
        _ => {
            ui.label(text);
        }
    }
}
