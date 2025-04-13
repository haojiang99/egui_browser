// src/app.rs
use crate::html_renderer::HtmlRenderer;
use crate::style::create_default_styles;
use crate::ui_components;
use eframe::egui;
use egui::Context;
use poll_promise::Promise;
use std::sync::{Arc, Mutex};

// Store the clicked link URL
#[derive(Clone, Default)]
pub struct LinkHandler {
    pub clicked_link: Arc<Mutex<Option<String>>>,
}

impl LinkHandler {
    pub fn new() -> Self {
        Self {
            clicked_link: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_link(&self, url: String) {
        let mut link = self.clicked_link.lock().unwrap();
        *link = Some(url);
    }

    pub fn take_link(&self) -> Option<String> {
        let mut link = self.clicked_link.lock().unwrap();
        link.take()
    }
}

// Navigation history structure
struct NavigationHistory {
    history: Vec<String>,
    current_index: usize,
}

impl NavigationHistory {
    fn new(initial_url: String) -> Self {
        Self {
            history: vec![initial_url],
            current_index: 0,
        }
    }

    fn can_go_back(&self) -> bool {
        self.current_index > 0
    }

    fn can_go_forward(&self) -> bool {
        self.current_index < self.history.len() - 1
    }

    fn go_back(&mut self) -> Option<&str> {
        if self.can_go_back() {
            self.current_index -= 1;
            Some(&self.history[self.current_index])
        } else {
            None
        }
    }

    fn go_forward(&mut self) -> Option<&str> {
        if self.can_go_forward() {
            self.current_index += 1;
            Some(&self.history[self.current_index])
        } else {
            None
        }
    }

    fn add_url(&mut self, url: String) {
        // Remove any forward history
        if self.current_index < self.history.len() - 1 {
            self.history.truncate(self.current_index + 1);
        }
        
        // Don't add if it's the same as the current URL
        if self.current_url() == url {
            return;
        }
        
        self.history.push(url);
        self.current_index = self.history.len() - 1;
    }

    fn current_url(&self) -> String {
        self.history[self.current_index].clone()
    }
}

// Our application state
pub struct EguiBrowser {
    url: String,
    html_content: Option<String>,
    error_message: Option<String>,
    // Promise to store the ongoing HTTP request
    fetch_promise: Option<Promise<Result<ehttp::Response, String>>>,
    // HTML renderer with styling
    html_renderer: HtmlRenderer,
    // State for showing/hiding raw HTML
    show_raw_html: bool,
    // Link handler for clicked links
    link_handler: LinkHandler,
    // Navigation history
    navigation: NavigationHistory,
    // User agent string
    user_agent: String,
}

impl Default for EguiBrowser {
    fn default() -> Self {
        let initial_url = "https://example.com".to_string();
        let link_handler = LinkHandler::new();
        let firefox_user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:125.0) Gecko/20100101 Firefox/125.0".to_string();
        Self {
            url: initial_url.clone(),
            html_content: None,
            error_message: None,
            fetch_promise: None,
            html_renderer: HtmlRenderer::new(create_default_styles(), link_handler.clone()),
            show_raw_html: false,
            link_handler,
            navigation: NavigationHistory::new(initial_url),
            user_agent: firefox_user_agent,
        }
    }
}

impl eframe::App for EguiBrowser {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Check if a link was clicked and handle it
        if let Some(link_url) = self.link_handler.take_link() {
            self.url = link_url.clone();
            self.navigation.add_url(link_url);
            self.fetch_url(ctx.clone());
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            
            
            // URL input field with navigation buttons
            ui.horizontal(|ui| {
                // Back button
                if ui.add_enabled(self.navigation.can_go_back(), egui::Button::new("←")).clicked() {
                    if let Some(url) = self.navigation.go_back() {
                        self.url = url.to_string();
                        self.fetch_url(ctx.clone());
                    }
                }
                
                // Forward button
                if ui.add_enabled(self.navigation.can_go_forward(), egui::Button::new("→")).clicked() {
                    if let Some(url) = self.navigation.go_forward() {
                        self.url = url.to_string();
                        self.fetch_url(ctx.clone());
                    }
                }
                
                ui.label("URL:");
                let response = ui.text_edit_singleline(&mut self.url);
                
                // Load button
                if ui.button("Load").clicked() {
                    self.navigation.add_url(self.url.clone());
                    self.fetch_url(ctx.clone());
                }
                
                // Automatically load when Enter is pressed in the text field
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.navigation.add_url(self.url.clone());
                    self.fetch_url(ctx.clone());
                }
            });
            
            // User agent options
            ui.horizontal(|ui| {
                ui.label("User Agent:");
                if ui.button("Firefox").clicked() {
                    self.user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:125.0) Gecko/20100101 Firefox/125.0".to_string();
                }
                if ui.button("Chrome").clicked() {
                    self.user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36".to_string();
                }
                if ui.button("Safari").clicked() {
                    self.user_agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4 Safari/605.1.15".to_string();
                }
                if ui.button("Edge").clicked() {
                    self.user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 Edg/124.0.0.0".to_string();
                }
            });
            
            // Show current user agent
            ui.label(format!("Current: {}", self.user_agent));
            
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
                // First display rendered HTML
                match html_parser::Dom::parse(html) {
                    Ok(dom) => {
                        ui_components::render_html_content(ui, &dom, &self.html_renderer);
                    }
                    Err(err) => {
                        ui.colored_label(egui::Color32::RED, format!("Failed to parse HTML: {}", err));
                    }
                }
                
                // Then display raw HTML below with toggle
                ui_components::render_raw_html_view(ui, html, &mut self.show_raw_html);
            }
        });
    }
}

impl EguiBrowser {
    // Start a new HTTP request to fetch the URL
    fn fetch_url(&mut self, ctx: Context) {
        let url = self.url.clone();
        let user_agent = self.user_agent.clone();
        
        // Create request with Firefox user agent
        let mut request = ehttp::Request::get(&url);
        request.headers.insert("User-Agent".to_string(), user_agent);
        
        let ctx_clone = ctx.clone();
        let promise = Promise::spawn_thread("fetch_url", move || {
            let result = ehttp::fetch_blocking(&request);
            ctx_clone.request_repaint();
            result
        });
        
        self.fetch_promise = Some(promise);
    }
}