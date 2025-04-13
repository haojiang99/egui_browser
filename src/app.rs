// src/app.rs
use crate::html_renderer::HtmlRenderer;
use crate::style::create_default_styles;
use crate::ui_components;
use eframe::egui;
use egui::Context;
use poll_promise::Promise;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::io::Read;

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
    // Image cache: URL -> (texture_id, size)
    image_cache: HashMap<String, (egui::TextureId, egui::Vec2)>,
    // Current image fetching promises
    image_promises: HashMap<String, Promise<Result<ehttp::Response, String>>>,
}

impl Default for EguiBrowser {
    fn default() -> Self {
        let initial_url = "http://web.simmons.edu/~grovesd/comm244/notes/week3/html-test-page.html".to_string();
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
            image_cache: HashMap::new(),
            image_promises: HashMap::new(),
        }
    }
}

impl EguiBrowser {
    // Helper method to create navigation arrow buttons with text-based arrows
    fn nav_button(&self, ui: &mut egui::Ui, text: &str, enabled: bool) -> bool {
        let arrow_btn = egui::Button::new(
            egui::RichText::new(text)
                .size(16.0)
                .strong()
        )
        .min_size(egui::vec2(40.0, 28.0));
        
        ui.add_enabled(enabled, arrow_btn).clicked()
    }

    // We'll retain this method but not apply it globally
    // so UI stays dark but web content can be white
    fn _configure_light_style(&self, ctx: &Context) {
        let mut style = (*ctx.style()).clone();
        
        // Set background colors to white
        style.visuals.panel_fill = egui::Color32::from_rgb(255, 255, 255);
        style.visuals.window_fill = egui::Color32::from_rgb(255, 255, 255);
        style.visuals.extreme_bg_color = egui::Color32::from_rgb(255, 255, 255);
        
        // Set text/UI elements to dark
        style.visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(33, 33, 33);
        style.visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(33, 33, 33);
        style.visuals.widgets.hovered.fg_stroke.color = egui::Color32::from_rgb(33, 33, 33);
        style.visuals.widgets.active.fg_stroke.color = egui::Color32::from_rgb(33, 33, 33);
        
        ctx.set_style(style);
    }
}

impl eframe::App for EguiBrowser {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Process any loaded images 
        self.process_images(ctx);
        
        // Set browser reference in renderer using a safer approach
        let browser_ptr = self as *const Self;
        self.html_renderer.browser = Some(browser_ptr);
        
        // On first frame, load the initial URL
        static mut FIRST_RUN: bool = true;
        unsafe {
            if FIRST_RUN {
                FIRST_RUN = false;
                self.fetch_url(ctx.clone());
            }
        }
        
        // Check if a link was clicked and handle it
        if let Some(link_url) = self.link_handler.take_link() {
            self.url = link_url.clone();
            self.navigation.add_url(link_url);
            self.fetch_url(ctx.clone());
        }

        // Use default (dark) frame for the UI elements
        egui::CentralPanel::default().show(ctx, |ui| {
            
            
            // URL input field with navigation buttons
            ui.horizontal(|ui| {
                // Back button with text-based arrow
                if self.nav_button(ui, "<-", self.navigation.can_go_back()) {
                    if let Some(url) = self.navigation.go_back() {
                        self.url = url.to_string();
                        self.fetch_url(ctx.clone());
                    }
                }
                
                // Forward button with text-based arrow
                if self.nav_button(ui, "->", self.navigation.can_go_forward()) {
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
                                    // Preprocess the HTML to remove problematic content
                                    let processed_html = self.preprocess_html(&text);
                                    self.html_content = Some(processed_html);
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
    // Preprocess HTML to remove scripts, styles, and simplify structure
    fn preprocess_html(&self, html: &str) -> String {
        // Check if the HTML is too large
        if html.len() > 1_000_000 {
            // For very large HTML, do a more aggressive truncation
            let truncated = &html[0..500_000];
            if let Some(end_pos) = truncated.rfind("</div>") {
                // Just return a simplified version
                return format!("<html><body><h1>Page content simplified</h1><p>The page was too large to display fully.</p>{}</body></html>", 
                              &truncated[0..end_pos+6]);
            } else {
                return "<html><body><h1>Page too large</h1><p>The page was too large to display.</p></body></html>".to_string();
            }
        }
        
        // Use a more efficient approach for large HTML
        let mut processed = String::with_capacity(html.len() / 2);
        let mut in_script = false;
        let mut in_style = false;
        let mut skip_until_index = 0;
        
        // Process the HTML in a single pass
        let chars: Vec<char> = html.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            if i < skip_until_index {
                i += 1;
                continue;
            }
            
            // Check for script start
            if !in_script && !in_style && i + 7 < chars.len() && 
               &chars[i..i+7].iter().collect::<String>() == "<script" {
                in_script = true;
                // Find the script end
                let rest = chars[i..].iter().collect::<String>();
                if let Some(end_pos) = rest.find("</script>") {
                    skip_until_index = i + end_pos + 9;
                    i += 1;
                    continue;
                }
            }
            
            // Check for style start
            if !in_script && !in_style && i + 6 < chars.len() && 
               &chars[i..i+6].iter().collect::<String>() == "<style" {
                in_style = true;
                // Find the style end
                let rest = chars[i..].iter().collect::<String>();
                if let Some(end_pos) = rest.find("</style>") {
                    skip_until_index = i + end_pos + 8;
                    i += 1;
                    continue;
                }
            }
            
            // Skip script and style content
            if in_script {
                if i + 9 < chars.len() && &chars[i..i+9].iter().collect::<String>() == "</script>" {
                    in_script = false;
                    i += 9;
                } else {
                    i += 1;
                }
                continue;
            }
            
            if in_style {
                if i + 8 < chars.len() && &chars[i..i+8].iter().collect::<String>() == "</style>" {
                    in_style = false;
                    i += 8;
                } else {
                    i += 1;
                }
                continue;
            }
            
            // Add current character to processed output
            processed.push(chars[i]);
            i += 1;
        }
        
        processed
    }

    // Start a new HTTP request to fetch the URL with timeout
    fn fetch_url(&mut self, ctx: Context) {
        let url = self.url.clone();
        let user_agent = self.user_agent.clone();
        
        // Create request with user agent
        let mut request = ehttp::Request::get(&url);
        request.headers.insert("User-Agent".to_string(), user_agent);
        
        // Add a timeout to prevent freezing
        let promise = Promise::spawn_thread("fetch_url", move || {
            // Use a more robust fetching approach with timeout
            let client = ureq::builder()
                .timeout_connect(std::time::Duration::from_secs(5))
                .timeout_read(std::time::Duration::from_secs(10))
                .build();
            
            match client.get(&url)
                    .set("User-Agent", &request.headers.get("User-Agent").unwrap_or(&String::new()))
                    .call() {
                Ok(response) => {
                    // Save response status before consuming the response
                    let status = response.status();
                    let status_text = response.status_text().to_string();
                    
                    // Read response body with size limit
                    let mut bytes = Vec::new();
                    // Limit to 2MB to prevent memory issues
                    const MAX_SIZE: usize = 2 * 1024 * 1024;
                    let mut reader = response.into_reader();
                    let mut buffer = [0; 8192];
                    let mut total_read = 0;
                    
                    loop {
                        match reader.read(&mut buffer) {
                            Ok(0) => break, // EOF
                            Ok(n) => {
                                total_read += n;
                                if total_read <= MAX_SIZE {
                                    bytes.extend_from_slice(&buffer[..n]);
                                } else {
                                    // We've read enough, stop here
                                    break;
                                }
                            }
                            Err(_) => return Err("Error reading response".to_string()),
                        }
                    }
                    
                    // Create a simplified response if too large
                    if total_read > MAX_SIZE {
                        bytes = "<html><body><h1>Content truncated</h1><p>The page was too large to display fully.</p></body></html>"
                            .as_bytes()
                            .to_vec();
                    }
                    
                    // Create ehttp response with all required fields
                    Ok(ehttp::Response {
                        url: url.clone(),
                        status,
                        status_text,
                        bytes,
                        ok: status >= 200 && status < 300,
                        headers: Default::default(), // Use an empty default header map
                    })
                }
                Err(err) => {
                    Err(format!("Failed to fetch URL: {}", err))
                }
            }
        });
        
        self.fetch_promise = Some(promise);
        ctx.request_repaint(); // Request a repaint to show the spinner
    }
    
    // Fetch image from URL and add to cache
    pub fn fetch_image(&mut self, ctx: &Context, image_url: String) {
        // Skip if already fetching or in cache
        if self.image_cache.contains_key(&image_url) || self.image_promises.contains_key(&image_url) {
            return;
        }
        
        // Resolve relative URLs
        let full_url = if image_url.starts_with("http") {
            image_url.clone()
        } else if image_url.starts_with("//") {
            // Protocol-relative URL (//example.com/image.png)
            if self.url.starts_with("https") {
                format!("https:{}", image_url)
            } else {
                format!("http:{}", image_url)
            }
        } else if image_url.starts_with('/') {
            // Absolute path from domain root
            let base_url = self.url.clone();
            // Extract domain with protocol (http://example.com)
            if let Some(protocol_end) = base_url.find("://") {
                // Extract domain part
                if let Some(domain_end) = base_url[protocol_end+3..].find('/') {
                    format!("{}{}", &base_url[..protocol_end+3+domain_end], image_url)
                } else {
                    // No path component in base URL
                    format!("{}{}", base_url, image_url)
                }
            } else {
                // Fallback if URL doesn't have protocol
                format!("{}{}", base_url, image_url)
            }
        } else {
            // Relative path
            let base_url = self.url.clone();
            if let Some(last_slash) = base_url.rfind('/') {
                // Make sure we're not just getting the protocol slashes
                if last_slash > 8 {  // Beyond "http://" or "https://"
                    format!("{}/{}", &base_url[..last_slash], image_url)
                } else {
                    // Append to the domain
                    format!("{}/{}", base_url, image_url)
                }
            } else {
                format!("{}/{}", base_url, image_url)
            }
        };
        
        // Create the request with the user agent
        let mut request = ehttp::Request::get(&full_url);
        request.headers.insert("User-Agent".to_string(), self.user_agent.clone());
        
        let ctx_clone = ctx.clone();
        let promise = Promise::spawn_thread("fetch_image", move || {
            let result = ehttp::fetch_blocking(&request);
            ctx_clone.request_repaint();
            result
        });
        
        self.image_promises.insert(image_url, promise);
    }
    
    // Process loaded images and add to texture cache
    fn process_images(&mut self, ctx: &Context) {
        let mut completed_urls = Vec::new();
        
        // Check all image promises
        for (url, promise) in &self.image_promises {
            if let Some(result) = promise.ready() {
                completed_urls.push(url.clone());
                
                match result {
                    Ok(response) => {
                        // Try to load the image
                        if let Ok(image) = image::load_from_memory(&response.bytes) {
                            let image = image.to_rgba8();
                            let dimensions = image.dimensions();
                            let image_data = egui::ColorImage::from_rgba_unmultiplied(
                                [dimensions.0 as usize, dimensions.1 as usize],
                                &image.into_raw(),
                            );
                            
                            // Add to texture cache
                            let texture = ctx.load_texture(
                                url.clone(),
                                image_data,
                                Default::default(),
                            );
                            
                            self.image_cache.insert(
                                url.clone(),
                                (texture.id(), egui::Vec2::new(dimensions.0 as f32, dimensions.1 as f32)),
                            );
                        }
                    }
                    Err(_) => {
                        // Image loading failed - ignore for now
                    }
                }
            }
        }
        
        // Remove completed promises
        for url in completed_urls {
            self.image_promises.remove(&url);
        }
    }
    
    // Get image texture if available
    pub fn get_image(&self, url: &str) -> Option<(egui::TextureId, egui::Vec2)> {
        self.image_cache.get(url).copied()
    }
}