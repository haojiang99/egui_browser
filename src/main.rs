use eframe::egui;
use egui::{ScrollArea, TextEdit};
use poll_promise::Promise;

// Our application state
struct EguiBrowser {
    url: String,
    html_content: Option<String>,
    error_message: Option<String>,
    // Promise to store the ongoing HTTP request
    fetch_promise: Option<Promise<Result<ehttp::Response, String>>>,
}

impl Default for EguiBrowser {
    fn default() -> Self {
        Self {
            url: "https://example.com".to_string(),
            html_content: None,
            error_message: None,
            fetch_promise: None,
        }
    }
}

impl eframe::App for EguiBrowser {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Simple HTML Browser");
            
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
                ui.separator();
                ui.heading("HTML Content:");
                
                ScrollArea::vertical().show(ui, |ui| {
                    // Display raw HTML (in a real browser we would render it properly)
                    let mut text = html.clone();
                    ui.add(TextEdit::multiline(&mut text).desired_width(f32::INFINITY));
                    
                    // Display a simplified rendering of the HTML
                    ui.separator();
                    ui.heading("Simple Rendered View:");
                    
                    match html_parser::Dom::parse(html) {
                        Ok(dom) => {
                            self.render_html_node(ui, &dom.children);
                        }
                        Err(err) => {
                            ui.colored_label(egui::Color32::RED, format!("Failed to parse HTML: {}", err));
                        }
                    }
                });
            }
        });
    }
}

impl EguiBrowser {
    // Start a new HTTP request to fetch the URL
    fn fetch_url(&mut self, ctx: egui::Context) {
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
    
    // A very simple HTML renderer
    fn render_html_node(&self, ui: &mut egui::Ui, nodes: &[html_parser::Node]) {
        for node in nodes {
            match node {
                html_parser::Node::Text(text) => {
                    ui.label(text);
                }
                html_parser::Node::Element(element) => {
                    match element.name.as_str() {
                        "h1" => {
                            ui.heading(self.get_text_content(&element.children));
                        }
                        "h2" => {
                            ui.heading(self.get_text_content(&element.children));
                        }
                        "p" => {
                            ui.label(self.get_text_content(&element.children));
                        }
                        "a" => {
                            if let Some(href) = element.attributes.get("href") {
                                if ui.link(self.get_text_content(&element.children)).clicked() {
                                    // In a real implementation, we would handle link clicks
                                    println!("Link clicked: {:?}", href);
                                }
                            } else {
                                ui.label(self.get_text_content(&element.children));
                            }
                        }
                        "div" | "span" | "body" => {
                            // Recursively render children
                            self.render_html_node(ui, &element.children);
                        }
                        _ => {
                            // Default handling for other elements
                            self.render_html_node(ui, &element.children);
                        }
                    }
                }
                // Ignore comments and other node types
                _ => {}
            }
        }
    }
    
    // Extract text content from HTML nodes
    fn get_text_content(&self, nodes: &[html_parser::Node]) -> String {
        let mut text = String::new();
        for node in nodes {
            match node {
                html_parser::Node::Text(t) => {
                    text.push_str(t);
                }
                html_parser::Node::Element(element) => {
                    text.push_str(&self.get_text_content(&element.children));
                }
                _ => {}
            }
        }
        text
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "egui Browser",
        options,
        Box::new(|_cc| Box::new(EguiBrowser::default())),
    )
}