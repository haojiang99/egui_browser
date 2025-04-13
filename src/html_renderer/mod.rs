// src/html_renderer/mod.rs
mod renderer;
mod style_handler;
mod text_processor;
mod renderers;
// Removing unused module: mod element_renderers;

pub use renderer::HtmlRenderer;
