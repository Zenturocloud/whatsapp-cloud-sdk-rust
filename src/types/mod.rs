//! Type definitions for the WhatsApp Cloud SDK
//!
//! This module contains all the data structures used for requests and responses
//! in the WhatsApp Cloud API.

pub mod messages;
pub mod media;
pub mod templates;
pub mod profile;
pub mod webhook;
pub mod business;


pub use messages::{
    SendTextMessage,
    SendMediaMessage,
    SendLocationMessage,
    SendTemplateMessage,
    SendInteractiveMessage,
    SendContactMessage,
    SendReactionMessage,
    MarkMessageAsRead,
    SendMessageResponse,
    MediaType,
    Component,
    Parameter,
    Interactive,
    InteractiveType,
    InteractiveAction,
    InteractiveButtons,
    InteractiveButton,
    InteractiveButtonType,
    InteractiveButtonReply,
    InteractiveBody,
    Contact,
};

pub use media::{
    UploadMedia,
    RetrieveMediaUrl,
    DeleteMedia,
    UploadMediaResponse,
    RetrieveMediaUrlResponse,
};

pub use templates::{
    GetTemplates,
    CreateTemplate,
    DeleteTemplate,
    GetTemplatesResponse,
    CreateTemplateResponse,
    TemplateComponent,
    TemplateComponentType,
    TemplateButton,
    TemplateButtonType,
};

pub use profile::{
    BusinessProfile,
    BusinessProfileResponse,
    RetrievePhoneNumbersResponse,
    RegisterPhoneNumber,
    DeregisterPhoneNumber,
    UpdatePhoneNumberSettings,
    PhoneNumberResponse,
};

pub use webhook::{
    WebhookEvent,
    WebhookMessage,
    WebhookMessageType,
    WebhookStatus,
    WebhookStatusType,
};

pub use business::{
    BusinessInfo,
    WhatsAppBusinessAccountResponse,
    CreateBusiness,
    GetBusinessAssets,
    BusinessAssetsResponse,
    WhatsAppBusinessAccountParams,
    AssignUserToWhatsAppBusiness,
    BusinessPhoneNumbersResponse,
    SystemUsersResponse,
    CreateSystemUser,
};
