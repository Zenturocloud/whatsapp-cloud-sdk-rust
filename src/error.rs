//! Error handling for the WhatsApp Cloud SDK

use std::fmt;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum WhatsAppError {
 
    #[error("WhatsApp API error: {message} (type: {error_type}, code: {code})")]
    ApiError {
        
        message: String,
      
        error_type: String,
       
        code: i32,
       
        subcode: Option<i32>,
       
        fbtrace_id: String,
        
        solution: Option<String>,
    },

  
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

  
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

 
    #[error("Rate limit exceeded. Try again in {retry_after_secs} seconds")]
    RateLimitExceeded {
        /// Number of seconds to wait before retrying
        retry_after_secs: u64,
    },


    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

 
    #[error("Validation error: {0}")]
    ValidationError(String),

   
    #[error("Missing required field: {0}")]
    MissingField(String),


    #[error("{0}")]
    Other(String),
}

pub struct ErrorHandler;

impl ErrorHandler {
    
    pub fn get_solution(error_type: &str, code: i32) -> Option<String> {
        let error_key = format!("{}-{}", error_type, code);
        
        match error_key.as_str() {
           
            "OAuthException-190" => Some("Check that your access token is valid and has not expired. You may need to generate a new one.".to_string()),
            "OAuthException-10" => Some("Ensure your app has the required permissions. Check your app settings in the Meta Developer Portal.".to_string()),
            
          
            "OAuthException-80004" => Some("Your application is making too many requests. Implement rate limiting or exponential backoff.".to_string()),
            "4-30" => Some("You have exceeded the rate at which you can send messages to this user. Wait and try again later.".to_string()),
            
           
            "GraphMethodException-100" => Some("One or more parameters in your request are invalid. Check the error details for specific fields to fix.".to_string()),
            "131000" => Some("The message failed to send. Check that the recipient is a valid WhatsApp user and try again.".to_string()),
            "131005" => Some("Your message contains content that is blocked by WhatsApp. Modify your message and try again.".to_string()),
            "131014" => Some("The template you are trying to use has not been approved. Check the status of your template in the Meta Business Manager.".to_string()),
            
            
            "131009" => Some("The media upload failed. Ensure the file is a supported format and size (images < 5MB, videos < 16MB, documents < 100MB).".to_string()),
            "131051" => Some("The media file you are trying to send could not be found. Check the media ID or URL.".to_string()),
            
            
            "132000" => Some("The phone number you are trying to use is not enabled for WhatsApp Business API. Verify the number in Meta Business Manager.".to_string()),
            "132001" => Some("The recipient phone number is not a verified WhatsApp user. Ensure the number is correct and the user has WhatsApp installed.".to_string()),
            
            // Default error
            _ => None,
        }
    }

    
    pub fn enhance_error(
        message: String,
        error_type: String,
        code: i32,
        subcode: Option<i32>,
        fbtrace_id: String,
    ) -> WhatsAppError {
        let solution = Self::get_solution(&error_type, code);
        
        WhatsAppError::ApiError {
            message,
            error_type,
            code,
            subcode,
            fbtrace_id,
            solution,
        }
    }
}


pub type WhatsAppResult<T> = Result<T, WhatsAppError>;
