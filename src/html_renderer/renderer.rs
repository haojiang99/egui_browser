// src/html_renderer/renderer.rs
use crate::style::ElementStyle;
use crate::app::{LinkHandler, EguiBrowser};
use egui::Ui;
use std::collections::HashMap;

// Import specific render functions from their modules
use crate::html_renderer::renderers::text::{render_heading, render_paragraph, render_link, render_text_formatting};
use crate::html_renderer::renderers::lists::render_list;
use crate::html_renderer::renderers::media::render_image;
use crate::html_renderer::renderers::block::{render_block_element, render_code};
use crate::html_renderer::renderers::forms::render_form_element;
use crate::html_renderer::renderers::tables::render_table;
use super::text_processor::get_text_content;

pub struct HtmlRenderer {
    pub style_map: HashMap<String, ElementStyle>,
    pub link_handler: LinkHandler,
    pub browser: Option<*const EguiBrowser>,
}

impl HtmlRenderer {
    pub fn new(style_map: HashMap<String, ElementStyle>, link_handler: LinkHandler) -> Self {
        Self { 
            style_map, 
            link_handler,
            browser: None,
        }
    }
    
    pub fn set_browser(&mut self, browser: &EguiBrowser) {
        self.browser = Some(browser as *const EguiBrowser);
    }
    
    pub fn get_browser(&self) -> Option<&EguiBrowser> {
        unsafe {
            self.browser.map(|ptr| &*ptr)
        }
    }
    
    // Find the body element in the DOM and filter out script/style content
    pub fn find_body_element<'a>(&self, nodes: &'a [html_parser::Node]) -> Option<Vec<html_parser::Node>> {
        for node in nodes {
            if let html_parser::Node::Element(element) = node {
                if element.name.to_lowercase() == "body" {
                    // Filter out script and style elements
                    let filtered_children = self.filter_nodes(&element.children);
                    return Some(filtered_children);
                }
                
                // Recursively search in children
                if let Some(body) = self.find_body_element(&element.children) {
                    return Some(body);
                }
            }
        }
        None
    }
    
    // Filter out script and style elements, and simplify deeply nested divs
    pub fn filter_nodes(&self, nodes: &[html_parser::Node]) -> Vec<html_parser::Node> {
        let mut filtered = Vec::new();
        
        for node in nodes {
            match node {
                html_parser::Node::Element(element) => {
                    let tag_name = element.name.to_lowercase();
                    
                    // Skip script and style elements completely
                    if tag_name == "script" || tag_name == "style" || tag_name == "noscript" {
                        continue;
                    }
                    
                    // Handle deeply nested div structures by flattening when possible
                    if tag_name == "div" && element.children.len() == 1 {
                        if let Some(html_parser::Node::Element(child)) = element.children.get(0) {
                            if child.name.to_lowercase() == "div" {
                                // Add the child's children directly, skipping one layer
                                let grandchildren = self.filter_nodes(&child.children);
                                filtered.extend(grandchildren);
                                continue;
                            }
                        }
                    }
                    
                    // For other elements, filter their children
                    let mut cloned = element.clone();
                    cloned.children = self.filter_nodes(&element.children);
                    filtered.push(html_parser::Node::Element(cloned));
                },
                html_parser::Node::Text(text) => {
                    // Keep only non-empty text nodes
                    let trimmed = text.trim();
                    if !trimmed.is_empty() {
                        filtered.push(html_parser::Node::Text(trimmed.to_string()));
                    }
                },
                _ => {} // Skip comments and other node types
            }
        }
        
        filtered
    }
    
    // Main HTML renderer
    pub fn render_html_node(&self, ui: &mut Ui, nodes: &[html_parser::Node]) {
        for node in nodes {
            
            match node {
                html_parser::Node::Text(text) => {
                    // Render plain text
                    let trimmed_text = text.trim();
                    if !trimmed_text.is_empty() {
                        ui.label(trimmed_text);
                    }
                }
                html_parser::Node::Element(element) => {
                    let tag_name = element.name.to_lowercase();
                    
                    // Get styling for the tag
                    let style = self.style_map.get(&tag_name);
                    
                    match tag_name.as_str() {
                        // Heading elements
                        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                            render_heading(ui, element, style, self);
                        }
                        
                        // Paragraph
                        "p" => {
                            render_paragraph(ui, element, style, self);
                        }
                        
                        // Links
                        "a" => {
                            render_link(ui, element, self);
                        }
                        
                        // Text formatting
                        "strong" | "b" | "em" | "i" => {
                            render_text_formatting(ui, element, tag_name.as_str(), self);
                        }
                        
                        // Lists
                        "ul" | "ol" => {
                            render_list(ui, element, tag_name.as_str(), self);
                        }
                        
                        // List items are handled in the list rendering functions
                        "li" => {
                            // This should be handled by parent ul/ol
                            let text = get_text_content(&element.children);
                            ui.label(text);
                        }
                        
                        // Image
                        "img" => {
                            render_image(ui, element, self);
                        }
                        
                        // Horizontal rule
                        "hr" => {
                            ui.add_space(4.0);
                            ui.separator();
                            ui.add_space(4.0);
                        }
                        
                        // Block elements
                        "div" | "section" | "article" | "main" | "aside" | "header" | "footer" => {
                            render_block_element(ui, element, self);
                        }
                        
                        // Inline elements - just render children
                        "span" => {
                            self.render_html_node(ui, &element.children);
                        }
                        
                        // Container elements - render their children
                        "html" | "body" | "head" => {
                            self.render_html_node(ui, &element.children);
                        }
                        
                        // Code blocks
                        "pre" | "code" => {
                            render_code(ui, element, self);
                        }
                        
                        // Form elements
                        "input" | "textarea" | "button" | "select" => {
                            render_form_element(ui, element, tag_name.as_str(), self);
                        }
                        
                        // Form container
                        "form" => {
                            ui.horizontal(|ui| {
                                self.render_html_node(ui, &element.children);
                            });
                        }
                        
                        // Table rendering
                        "table" => {
                            render_table(ui, element, self);
                        }
                        
                        // Any other element - render its children
                        _ => {
                            self.render_html_node(ui, &element.children);
                        }
                    }
                }
                // Ignore comments and other node types
                _ => {}
            }
        }
    }
}