# WhatsApp Cloud SDK for Rust

A comprehensive, developer-friendly Rust wrapper for the WhatsApp Cloud API. This SDK provides intuitive access to all WhatsApp Business Platform features with strong typing, detailed documentation, and helpful abstractions.

[![Crates.io Version](https://img.shields.io/crates/v/whatsapp-cloud-sdk.svg)](https://crates.io/crates/whatsapp-cloud-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/zenturocloud/whatsapp-cloud-sdk-rust/blob/main/LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70+-blueviolet.svg)](https://www.rust-lang.org/)

## Features

- **Complete API Coverage**: Access all WhatsApp Cloud API endpoints including messaging, templates, media, contacts, and more
- **Type Safety**: Leverages Rust's type system to provide comprehensive type definitions for all API responses and requests
- **Authentication Handling**: Simple setup for access tokens with automatic token refresh
- **Webhook Support**: Easy webhook configuration and event handling
- **Error Handling**: Detailed error responses with helpful troubleshooting guidance
- **Rate Limiting**: Built-in mechanisms to handle API rate limits gracefully
- **Media Handling**: Simplified uploading, downloading, and managing media files
- **Template Management**: Create, update, and send message templates with ease
- **Conversation Features**: Support for all messaging types (text, media, interactive, etc.)
- **Async/Await Support**: Full support for Rust's async/await paradigm
- **Zero-Cost Abstractions**: Idiomatic Rust with minimal runtime overhead

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
whatsapp-cloud-sdk = "1.0.0"
```

## Quick Start

```rust
use whatsapp_cloud_sdk::{create_client, messages::SendTextMessage};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let whatsapp = create_client(
        "YOUR_ACCESS_TOKEN",
        "YOUR_PHONE_NUMBER_ID",
        None, // Optional API version, defaults to latest
    );

    // Send a text message
    let response = whatsapp.send_text_message(SendTextMessage {
        to: "15551234567".to_string(),
        text: "Hello from WhatsApp Cloud SDK for Rust!".to_string(),
        preview_url: None,
    }).await?;

    println!("Message sent: {}", response.messages[0].id);
    
    Ok(())
}
```

## Documentation

For complete documentation, visit [our documentation site](#).

### Core Components

- Client Setup and Authentication
- Sending Messages (Text, Media, Interactive, etc.)
- Receiving and Processing Webhooks
- Managing Templates
- Handling Media
- Error Handling and Troubleshooting

## Examples

### Sending a Media Message

```rust
// Send an image
let response = whatsapp.send_media_message(SendMediaMessage {
    to: "15551234567".to_string(),
    media_type: MediaType::Image,
    media_url: Some("https://example.com/image.jpg".to_string()),
    media_id: None,
    caption: Some("Check out this image!".to_string()),
    filename: None,
}).await?;
```

### Sending an Interactive Message

```rust
// Send an interactive button message
let response = whatsapp.send_interactive_message(SendInteractiveMessage {
    to: "15551234567".to_string(),
    interactive: Interactive {
        r#type: InteractiveType::Button,
        body: InteractiveBody {
            text: "Would you like to proceed?".to_string(),
        },
        action: InteractiveAction::Buttons(InteractiveButtons {
            buttons: vec![
                InteractiveButton {
                    r#type: InteractiveButtonType::Reply,
                    reply: InteractiveButtonReply {
                        id: "yes".to_string(),
                        title: "Yes".to_string(),
                    },
                },
                InteractiveButton {
                    r#type: InteractiveButtonType::Reply,
                    reply: InteractiveButtonReply {
                        id: "no".to_string(),
                        title: "No".to_string(),
                    },
                },
            ],
        }),
        header: None,
        footer: None,
    },
}).await?;
```

### Handling Webhooks

```rust
use whatsapp_cloud_sdk::{create_webhook_handler, webhook::{WebhookEvent, WebhookMessage, WebhookStatus}};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle_webhook(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let webhook_handler = create_webhook_handler(Some("YOUR_APP_SECRET".to_string()), Some("YOUR_VERIFY_TOKEN".to_string()));
    
    match (req.method(), req.uri().path()) {
        // Handle GET request for webhook verification
        (&hyper::Method::GET, "/webhook") => {
            let query = req.uri().query().unwrap_or("");
            let params: Vec<(String, String)> = url::form_urlencoded::parse(query.as_bytes())
                .into_owned()
                .collect();
            
            let mode = params.iter().find(|(k, _)| k == "hub.mode").map(|(_, v)| v);
            let token = params.iter().find(|(k, _)| k == "hub.verify_token").map(|(_, v)| v);
            let challenge = params.iter().find(|(k, _)| k == "hub.challenge").map(|(_, v)| v);
            
            if let (Some(mode), Some(token), Some(challenge)) = (mode, token, challenge) {
                if let Some(response) = webhook_handler.verify_webhook(mode, token, challenge) {
                    return Ok(Response::new(Body::from(response)));
                }
            }
            
            Ok(Response::builder().status(403).body(Body::empty()).unwrap())
        },
        
        // Handle POST request for webhook events
        (&hyper::Method::POST, "/webhook") => {
            let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let signature = req.headers().get("x-hub-signature-256")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");
            
            // Validate signature
            if !webhook_handler.validate_signature(signature, &body_bytes) {
                return Ok(Response::builder().status(403).body(Body::empty()).unwrap());
            }
            
            // Parse and process the webhook event
            if let Ok(event) = serde_json::from_slice::<WebhookEvent>(&body_bytes) {
                webhook_handler.handle_webhook(event, |message: Option<WebhookMessage>, status: Option<WebhookStatus>| {
                    if let Some(message) = message {
                        println!("Received message: {:?}", message);
                        // Handle the message
                    }
                    
                    if let Some(status) = status {
                        println!("Message status update: {:?}", status);
                        // Handle the status update
                    }
                });
            }
            
            // Always respond with 200 OK quickly to acknowledge receipt
            Ok(Response::new(Body::from("OK")))
        },
        
        // Handle other requests
        _ => Ok(Response::builder().status(404).body(Body::empty()).unwrap()),
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_webhook))
    });
    
    let server = Server::bind(&addr).serve(make_svc);
    
    println!("Webhook server listening on http://{}", addr);
    
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
```

## Project Status

This is a closed-source project maintained by ZenturoCloud. While we welcome feedback, bug reports, and feature requests through the Issues section, we are not accepting code contributions at this time.

The SDK is professionally maintained and regularly updated to ensure compatibility with the latest WhatsApp Cloud API versions.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Maintenance & Version Support

This SDK is actively maintained with dedicated support for the following WhatsApp API versions:

- v19.0 (tag: `v19`)
- v20.0 (tag: `v20`)
- v21.0 (tag: `v21`) 
- v22.0 (tag: `v22`)

New API version tags will be added as Meta releases them, ensuring you always have access to the latest WhatsApp Cloud API features. Each version is thoroughly tested against its respective API version.

We're committed to long-term maintenance of this SDK with:
- Regular updates for new API features
- Security patches
- Bug fixes
- Performance improvements

## Related Projects

- [whatsapp-cloud-sdk-node.js](https://github.com/zenturocloud/whatsapp-cloud-sdk-node.js)
- [whatsapp-cloud-sdk-python](https://github.com/zenturocloud/whatsapp-cloud-sdk-python)
