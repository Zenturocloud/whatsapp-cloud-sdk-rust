//! WhatsApp Client for sending messages and managing media
//!
//! This module provides the main client for interacting with the WhatsApp Cloud API.

use reqwest::{Client as HttpClient, header};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::path::Path;

use crate::error::{WhatsAppError, WhatsAppResult, ErrorHandler};
use crate::rate_limiter::RateLimiter;
use crate::types::*;


#[derive(Clone, Debug)]
pub struct ClientConfig {
 
    pub access_token: String,
 
    pub phone_number_id: String,
   
    pub business_account_id: Option<String>,

    pub version: String,
  
    pub max_requests_per_minute: u32,
    
    pub retry_after_too_many_requests: bool,
 
    pub max_retries: u32,
   
    pub retry_delay_ms: u64,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            access_token: String::new(),
            phone_number_id: String::new(),
            business_account_id: None,
            version: "v22.0".to_string(),
            max_requests_per_minute: 250,
            retry_after_too_many_requests: true,
            max_retries: 3,
            retry_delay_ms: 1000,
        }
    }
}


#[derive(Clone)]
pub struct WhatsAppClient {
    config: ClientConfig,
    http_client: HttpClient,
    rate_limiter: Arc<RateLimiter>,
    base_url: String,
}

impl WhatsAppClient {

    pub fn new(config: ClientConfig) -> Self {
        let mut headers = header::HeaderMap::new();
        let auth_value = format!("Bearer {}", config.access_token);
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&auth_value).unwrap(),
        );
        
        let http_client = HttpClient::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to create HTTP client");
        
        let base_url = format!("https://graph.facebook.com/{}", config.version);
        
        let rate_limiter = Arc::new(RateLimiter::new(
            config.max_requests_per_minute,
            config.retry_after_too_many_requests,
            config.max_retries,
            config.retry_delay_ms,
        ));
        
        Self {
            config,
            http_client,
            rate_limiter,
            base_url,
        }
    }
    
   
    fn get_phone_number_url(&self) -> String {
        format!("/{}", self.config.phone_number_id)
    }
    
  
    fn get_messages_url(&self) -> String {
        format!("/{}/messages", self.config.phone_number_id)
    }
    
    fn get_media_url(&self) -> String {
        format!("/{}/media", self.config.phone_number_id)
    }
    
    pub fn update_access_token(&mut self, access_token: String) {
        self.config.access_token = access_token.clone();
        
        let mut headers = header::HeaderMap::new();
        let auth_value = format!("Bearer {}", access_token);
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&auth_value).unwrap(),
        );
        
        self.http_client = HttpClient::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to create HTTP client");
    }

    
}


pub fn create_client(
    access_token: &str,
    phone_number_id: &str,
    version: Option<&str>,
) -> WhatsAppClient {
    let config = ClientConfig {
        access_token: access_token.to_string(),
        phone_number_id: phone_number_id.to_string(),
        business_account_id: None,
        version: version.unwrap_or("v22.0").to_string(),
        max_requests_per_minute: 250,
        retry_after_too_many_requests: true,
        max_retries: 3,
        retry_delay_ms: 1000,
    };
    
    WhatsAppClient::new(config)
}
