//! WhatsApp Cloud SDK for Rust
//!
//! A comprehensive, developer-friendly Rust wrapper for the WhatsApp Cloud API.
//! This SDK provides intuitive access to all WhatsApp Business Platform features
//! with strong typing, detailed documentation, and helpful abstractions.

pub mod client;
pub mod business;
pub mod webhook;
pub mod rate_limiter;
pub mod error;
pub mod types;
pub mod util;

pub use client::{WhatsAppClient, ClientConfig, create_client};
pub use business::{BusinessClient, BusinessClientConfig, create_business_client};
pub use webhook::{WebhookHandler, WebhookConfig, create_webhook_handler};
