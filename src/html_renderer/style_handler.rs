// src/html_renderer/style_handler.rs
use crate::style::ElementStyle;
use egui::{Color32, RichText};

// Apply styling to RichText based on ElementStyle
pub fn apply_style(text: &str, style: Option<&ElementStyle>) -> RichText {
    let mut rich_text = RichText::new(text);
    
    if let Some(style) = style {
        if let Some(color) = style.color {
            rich_text = rich_text.color(color);
        }
        if let Some(size) = style.font_size {
            rich_text = rich_text.size(size);
        }
        if let Some(_weight) = style.font_weight {
            rich_text = rich_text.strong();
        }
    }
    
    rich_text
}

// Get link styling
pub fn get_link_style(text: &str) -> RichText {
    RichText::new(text)
        .color(Color32::from_rgb(0, 102, 204))
        .underline()
}
