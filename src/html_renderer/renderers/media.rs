// src/html_renderer/renderers/media.rs
use egui::Ui;
use crate::html_renderer::text_processor::get_attribute;
use crate::html_renderer::renderer::HtmlRenderer;
use crate::app::EguiBrowser;

// Render image
pub fn render_image(
    ui: &mut Ui, 
    element: &html_parser::Element,
    html_renderer: &HtmlRenderer
) {
    let src = get_attribute(element, "src", "");
    let alt = get_attribute(element, "alt", "");
    
    if src.is_empty() {
        // No source attribute, show placeholder
        ui.label("[No image source]");
        return;
    }
    
    // Check if we have the image in cache
    if let Some(browser) = html_renderer.get_browser() {
        if let Some((texture_id, size)) = browser.get_image(&src) {
            // Calculate a reasonable display size, respecting width/height if specified
            let width_attr = get_attribute(element, "width", "");
            let height_attr = get_attribute(element, "height", "");
            
            let mut display_size = size;
            
            // Apply width constraint if specified
            if !width_attr.is_empty() {
                if let Ok(width) = width_attr.parse::<f32>() {
                    let scale = width / size.x;
                    display_size = egui::Vec2::new(width, size.y * scale);
                }
            }
            
            // Apply height constraint if specified
            if !height_attr.is_empty() {
                if let Ok(height) = height_attr.parse::<f32>() {
                    let scale = height / size.y;
                    display_size = egui::Vec2::new(size.x * scale, height);
                }
            }
            
            // Limit maximum size to available width
            let available_width = ui.available_width();
            if display_size.x > available_width {
                let scale = available_width / display_size.x;
                display_size = egui::Vec2::new(available_width, display_size.y * scale);
            }
            
            // Display the image
            ui.add(egui::Image::new((texture_id, display_size)));
        } else {
            // Request the image to be fetched
            if ui.button("Load Image").clicked() {
                // Just need to request a repaint - the app will handle loading in the update loop
                ui.ctx().request_repaint();
            }
            
            // Show placeholder while loading
            if !alt.is_empty() {
                ui.label(format!("[Loading image: {}]", alt));
            } else {
                ui.label("[Loading image...]");
            }
        }
    } else {
        // No browser reference, show placeholder
        if !alt.is_empty() {
            ui.label(format!("[Image: {}]", alt));
        } else {
            ui.label("[Image]");
        }
    }
}
