// src/html_renderer/renderers/tables.rs
use egui::{RichText, Ui};
use crate::html_renderer::renderer::HtmlRenderer;
use crate::html_renderer::text_processor::get_text_content;

// Render table
pub fn render_table(
    ui: &mut Ui, 
    table_element: &html_parser::Element,
    _renderer: &HtmlRenderer
) {
    ui.add_space(4.0);
    ui.group(|ui| {
        // Extract and render table components
        let (headers, rows) = extract_table_data(table_element);
        
        // Render table header
        if !headers.is_empty() {
            ui.horizontal(|ui| {
                for header in headers {
                    ui.label(RichText::new(header).strong());
                }
            });
            ui.separator();
        }
        
        // Render table rows
        for row in rows {
            ui.horizontal(|ui| {
                for cell in row {
                    ui.label(cell);
                }
            });
        }
    });
    ui.add_space(4.0);
}

// Extract table data
fn extract_table_data(
    table_element: &html_parser::Element
) -> (Vec<String>, Vec<Vec<String>>) {
    let mut headers = Vec::new();
    let mut rows = Vec::new();
    
    for node in &table_element.children {
        if let html_parser::Node::Element(element) = node {
            match element.name.to_lowercase().as_str() {
                "thead" => {
                    for thead_child in &element.children {
                        if let html_parser::Node::Element(tr_element) = thead_child {
                            if tr_element.name.to_lowercase() == "tr" {
                                for th_node in &tr_element.children {
                                    if let html_parser::Node::Element(th_element) = th_node {
                                        if th_element.name.to_lowercase() == "th" {
                                            headers.push(get_text_content(&th_element.children));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                "tbody" => {
                    for tbody_child in &element.children {
                        if let html_parser::Node::Element(tr_element) = tbody_child {
                            if tr_element.name.to_lowercase() == "tr" {
                                let mut row = Vec::new();
                                for td_node in &tr_element.children {
                                    if let html_parser::Node::Element(td_element) = td_node {
                                        if td_element.name.to_lowercase() == "td" {
                                            row.push(get_text_content(&td_element.children));
                                        }
                                    }
                                }
                                if !row.is_empty() {
                                    rows.push(row);
                                }
                            }
                        }
                    }
                }
                "tr" => {
                    // Handle direct tr children (when no thead/tbody)
                    let mut row = Vec::new();
                    for td_node in &element.children {
                        if let html_parser::Node::Element(td_element) = td_node {
                            if td_element.name.to_lowercase() == "td" || td_element.name.to_lowercase() == "th" {
                                row.push(get_text_content(&td_element.children));
                            }
                        }
                    }
                    if !row.is_empty() {
                        rows.push(row);
                    }
                }
                _ => {}
            }
        }
    }
    
    (headers, rows)
}
