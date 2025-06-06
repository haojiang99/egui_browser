// src/style.rs
use egui::{Color32, Vec2};
use std::collections::HashMap;

// Structure to hold CSS-like styling properties
pub struct ElementStyle {
    pub color: Option<Color32>,
    pub font_size: Option<f32>,
    pub font_weight: Option<f32>,
    pub margin: Option<Vec2>,
    #[allow(dead_code)]
    pub padding: Option<Vec2>,
    #[allow(dead_code)]
    pub background_color: Option<Color32>,
}

impl Default for ElementStyle {
    fn default() -> Self {
        Self {
            color: None,
            font_size: None,
            font_weight: None,
            margin: None,
            padding: None,
            background_color: None,
        }
    }
}

pub fn create_default_styles() -> HashMap<String, ElementStyle> {
    let mut style_map = HashMap::new();
    
    // Define default styles for common HTML elements
    let mut body_style = ElementStyle::default();
    body_style.color = Some(Color32::from_rgb(33, 33, 33)); // Dark text
    body_style.background_color = Some(Color32::from_rgb(255, 255, 255)); // White background
    style_map.insert("body".to_string(), body_style);
    
    let mut h1_style = ElementStyle::default();
    h1_style.font_size = Some(28.0);
    h1_style.font_weight = Some(800.0);
    h1_style.margin = Some(Vec2::new(0.0, 10.0));
    h1_style.color = Some(Color32::from_rgb(33, 33, 33)); // Dark text
    style_map.insert("h1".to_string(), h1_style);
    
    let mut h2_style = ElementStyle::default();
    h2_style.font_size = Some(24.0);
    h2_style.font_weight = Some(700.0);
    h2_style.margin = Some(Vec2::new(0.0, 8.0));
    h2_style.color = Some(Color32::from_rgb(33, 33, 33)); // Dark text
    style_map.insert("h2".to_string(), h2_style);
    
    let mut h3_style = ElementStyle::default();
    h3_style.font_size = Some(20.0);
    h3_style.font_weight = Some(600.0);
    h3_style.margin = Some(Vec2::new(0.0, 6.0));
    h3_style.color = Some(Color32::from_rgb(33, 33, 33)); // Dark text
    style_map.insert("h3".to_string(), h3_style);
    
    let mut p_style = ElementStyle::default();
    p_style.margin = Some(Vec2::new(0.0, 4.0));
    p_style.color = Some(Color32::from_rgb(33, 33, 33)); // Dark text
    style_map.insert("p".to_string(), p_style);
    
    let mut a_style = ElementStyle::default();
    a_style.color = Some(Color32::from_rgb(0, 102, 204)); // Blue links
    style_map.insert("a".to_string(), a_style);
    
    let mut strong_style = ElementStyle::default();
    strong_style.font_weight = Some(700.0);
    strong_style.color = Some(Color32::from_rgb(33, 33, 33)); // Dark text
    style_map.insert("strong".to_string(), strong_style);
    
    let mut em_style = ElementStyle::default();
    em_style.font_weight = Some(400.0); // Normal weight but italic (handled in rendering)
    em_style.color = Some(Color32::from_rgb(33, 33, 33)); // Dark text
    style_map.insert("em".to_string(), em_style);
    
    // Add style for span elements
    let mut span_style = ElementStyle::default();
    span_style.color = Some(Color32::from_rgb(33, 33, 33)); // Dark text
    style_map.insert("span".to_string(), span_style);
    
    // Add style for div elements
    let mut div_style = ElementStyle::default();
    div_style.color = Some(Color32::from_rgb(33, 33, 33)); // Dark text
    div_style.background_color = Some(Color32::from_rgb(255, 255, 255)); // White background
    style_map.insert("div".to_string(), div_style);
    
    style_map
}