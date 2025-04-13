// src/app.rs
use crate::html_renderer::HtmlRenderer;
use crate::style::create_default_styles;
use crate::ui_components;
use eframe::egui;
use egui::Context;
use poll_promise::Promise;

// Our application state
pub struct EguiBrowser {
    url: String,
    html_content: Option<String>,
    error_message: Option<String>,
    // Promise to store the ongoing HTTP request
    fetch_promise: Option<Promise<Result<ehttp::Response, String>>>,
    // HTML renderer with styling
    html_renderer: HtmlRenderer,
}

impl Default for EguiBrowser {
    fn default() -> Self {
        Self {
            url: "https://example.com".to_string(),
            html_content: None,
            error_message: None,
            fetch_promise: None,
            html_renderer: HtmlRenderer::new(create_default_styles()),
        }
    }
}

impl eframe::App for EguiBrowser {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("HTML Browser");
            
            // URL input field
            ui.horizontal(|ui| {
                ui.label("URL:");
                let response = ui.text_edit_singleline(&mut self.url);
                
                // Load button
                if ui.button("Load").clicked() {
                    self.fetch_url(ctx.clone());
                }
                
                // Automatically load when Enter is pressed in the text field
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.fetch_url(ctx.clone());
                }
            });
            
            // Show error message if any
            if let Some(error) = &self.error_message {
                ui.colored_label(egui::Color32::RED, error);
            }
            
            // Check if the promise is complete
            if let Some(promise) = &self.fetch_promise {
                if let Some(result) = promise.ready() {
                    match result {
                        Ok(response) => {
                            // Try to convert the response bytes to a string
                            match String::from_utf8(response.bytes.clone()) {
                                Ok(text) => {
                                    self.html_content = Some(text);
                                    self.error_message = None;
                                }
                                Err(_) => {
                                    self.error_message = Some("Failed to decode response as UTF-8".to_string());
                                }
                            }
                            // Clear the promise
                            self.fetch_promise = None;
                        }
                        Err(err) => {
                            self.error_message = Some(format!("Error: {}", err));
                            self.fetch_promise = None;
                        }
                    }
                } else {
                    ui.spinner(); // Show a spinner while loading
                }
            }
            
            // Show HTML content
            if let Some(html) = &self.html_content {
                // Display raw HTML
                ui_components::render_raw_html_view(ui, html);
                
                // Display rendered HTML
                match html_parser::Dom::parse(html) {
                    Ok(dom) => {
                        ui_components::render_html_content(ui, &dom, &self.html_renderer);
                    }
                    Err(err) => {
                        ui.colored_label(egui::Color32::RED, format!("Failed to parse HTML: {}", err));
                    }
                }
            }
        });
    }
}

impl EguiBrowser {
    // Start a new HTTP request to fetch the URL
    fn fetch_url(&mut self, ctx: Context) {
        let url = self.url.clone();
        let request = ehttp::Request::get(&url);
        
        let ctx_clone = ctx.clone();
        let promise = Promise::spawn_thread("fetch_url", move || {
            let result = ehttp::fetch_blocking(&request);
            ctx_clone.request_repaint();
            result
        });
        
        self.fetch_promise = Some(promise);
    }
}
