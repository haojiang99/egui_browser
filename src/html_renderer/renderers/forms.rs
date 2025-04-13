// src/html_renderer/renderers/forms.rs
use egui::Ui;
use crate::html_renderer::renderer::HtmlRenderer;
use crate::html_renderer::text_processor::{get_text_content, get_attribute};

// Render form element
pub fn render_form_element(
    ui: &mut Ui, 
    element: &html_parser::Element,
    tag: &str,
    _renderer: &HtmlRenderer
) {
    match tag {
        "input" => render_input(ui, element),
        "textarea" => {
            let mut text = get_text_content(&element.children);
            ui.text_edit_multiline(&mut text);
        }
        "button" => {
            let text = get_text_content(&element.children);
            let _ = ui.button(text);
        }
        "select" => {
            ui.label("[Dropdown menu]");
        }
        _ => {}
    }
}

// Render input element
fn render_input(ui: &mut Ui, element: &html_parser::Element) {
    let input_type = get_attribute(element, "type", "text");
    let name = get_attribute(element, "name", "");
    
    match input_type.as_str() {
        "button" | "submit" => {
            let value = get_attribute(element, "value", "Button");
            if value.is_empty() {
                let _ = ui.button("Submit");
            } else {
                let _ = ui.button(&value);
            }
        }
        "checkbox" => {
            let mut checked = element.attributes.contains_key("checked");
            ui.checkbox(&mut checked, "");
        }
        "hidden" => {
            // Don't render hidden inputs
        }
        "search" | "text" | "password" | "email" | _ => {
            let mut value = get_attribute(element, "value", "");
            let placeholder = get_attribute(element, "placeholder", "");
            
            // Create more visible text field
            let field = egui::TextEdit::singleline(&mut value)
                .desired_width(200.0)
                .hint_text(if placeholder.is_empty() { &name } else { &placeholder });
                
            ui.add(field);
        }
    }
}
