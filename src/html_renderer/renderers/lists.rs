// src/html_renderer/renderers/lists.rs
use egui::Ui;
use crate::html_renderer::renderer::HtmlRenderer;

// Render list (ordered or unordered)
pub fn render_list(
    ui: &mut Ui, 
    element: &html_parser::Element,
    list_type: &str,
    renderer: &HtmlRenderer
) {
    ui.add_space(4.0);
    
    match list_type {
        "ul" => render_unordered_list(ui, &element.children, renderer),
        "ol" => render_ordered_list(ui, &element.children, renderer),
        _ => {}
    }
    
    ui.add_space(4.0);
}

// Render unordered list
fn render_unordered_list(
    ui: &mut Ui, 
    nodes: &[html_parser::Node],
    renderer: &HtmlRenderer
) {
    for node in nodes {
        if let html_parser::Node::Element(element) = node {
            if element.name.to_lowercase() == "li" {
                ui.horizontal(|ui| {
                    ui.label("• ");
                    ui.vertical(|ui| {
                        renderer.render_html_node(ui, &element.children);
                    });
                });
            } else {
                // For nested lists or other elements
                renderer.render_html_node(ui, &[node.clone()]);
            }
        }
    }
}

// Render ordered list
fn render_ordered_list(
    ui: &mut Ui, 
    nodes: &[html_parser::Node],
    renderer: &HtmlRenderer
) {
    let mut counter = 1;
    for node in nodes {
        if let html_parser::Node::Element(element) = node {
            if element.name.to_lowercase() == "li" {
                ui.horizontal(|ui| {
                    ui.label(format!("{}. ", counter));
                    counter += 1;
                    ui.vertical(|ui| {
                        renderer.render_html_node(ui, &element.children);
                    });
                });
            } else {
                // For nested lists or other elements
                renderer.render_html_node(ui, &[node.clone()]);
            }
        }
    }
}
