//! Types for sending and receiving WhatsApp messages

use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {

    Audio,

    Document,
 
    Image,
   
    Sticker,
 
    Video,
}


#[derive(Debug, Clone, Serialize)]
pub struct SendTextMessage {

    pub to: String,
 
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<bool>,
}


#[derive(Debug, Clone, Serialize)]
pub struct SendMediaMessage {
  
    pub to: String,

    pub media_type: MediaType,
 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_id: Option<String>,
 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
   
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
   
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}


#[derive(Debug, Clone, Serialize)]
pub struct SendLocationMessage {
  
    pub to: String,
  
    pub latitude: f64,

    pub longitude: f64,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ComponentType {

    Header,
  
    Body,
  
    Button,

    Footer,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Parameter {
   
    Text {
     
        text: String,
    },

    Currency {
        
        currency: Currency,
    },

    DateTime {
      
        date_time: DateTime,
    },
 
    Image {
        
        image: Image,
    },

    Document {
     
        document: Document,
    },
  
    Video {
        
        video: Video,
    },
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {

    pub code: String,

    pub amount: f64,
    
    pub fallback_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTime {
    
    pub fallback_value: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    
    pub link: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
  
    pub link: String,
 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
 
    pub link: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {

    pub r#type: ComponentType,

    pub parameters: Vec<Parameter>,
}


#[derive(Debug, Clone, Serialize)]
pub struct SendTemplateMessage {
  
    pub to: String,
 
    pub template_name: String,

    pub language_code: String,
 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,
}


#[derive(Debug, Clone, Serialize)]
pub struct SendTemplateMessageWithTtl {
   
    pub to: String,
   
    pub template_name: String,
 
    pub language_code: String,
  
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,
 
    pub ttl: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InteractiveType {
 
    Button,

    List,

    Product,
  
    ProductList,

    Flow,

    CtaUrl,
 
    LocationRequestMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveBody {
 
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InteractiveButtonType {
  
    Reply,
 
    Url,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveButtonReply {
    
    pub id: String,
    
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveButton {
 
    pub r#type: InteractiveButtonType,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply: Option<InteractiveButtonReply>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
   
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveButtons {
   
    pub buttons: Vec<InteractiveButton>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveSectionRow {

    pub id: String,
   
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveSection {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
   
    pub rows: Vec<InteractiveSectionRow>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveSections {
 
    pub sections: Vec<InteractiveSection>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveProductRetailers {
    
    pub product_retailers: Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveCatalog {
  
    pub catalog_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveFlow {
  
    pub id: String,
  
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractiveFlowData>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveFlowData {
 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InteractiveAction {
 
    Buttons(InteractiveButtons),
 
    Sections(InteractiveSections),

    ProductRetailers(InteractiveProductRetailers),
 
    Catalog(InteractiveCatalog),

    Flow(InteractiveFlowAction),

    LocationRequest(InteractiveLocationRequestAction),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveFlowAction {

    pub name: String,

    pub parameters: InteractiveFlowParameters,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveFlowParameters {

    pub flow_token: String,
 
    pub flow_id: String,
    
    pub flow_cta: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveLocationRequestAction {

    pub name: String,
 
    pub button: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InteractiveHeaderType {
 
    Text,
   
    Video,

    Image,

    Document,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InteractiveHeader {
  
    Text {
     
        text: String,
    },
  
    Video {
  
        video: InteractiveHeaderMedia,
    },

    Image {
      
        image: InteractiveHeaderMedia,
    },
 
    Document {
   
        document: InteractiveHeaderDocument,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveHeaderMedia {

    pub link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveHeaderDocument {

    pub link: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveFooter {

    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interactive {
  
    pub r#type: InteractiveType,

    pub body: InteractiveBody,

    pub action: InteractiveAction,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<InteractiveHeader>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<InteractiveFooter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<InteractiveFlow>,
}


#[derive(Debug, Clone, Serialize)]
pub struct SendInteractiveMessage {

    pub to: String,
   
    pub interactive: Interactive,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactAddress {
 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
   
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
   
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
  
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactEmail {
   
    pub email: String,
 
    pub r#type: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactName {
    
    pub formatted_name: String,
  
    pub first_name: String,
  
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
   
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactOrg {
   
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactPhone {
 
    pub phone: String,
 
    pub r#type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wa_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactUrl {
  
    pub url: String,
    
    pub r#type: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<ContactAddress>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<ContactEmail>>,
 
    pub name: ContactName,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub org: Option<ContactOrg>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phones: Option<Vec<ContactPhone>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<ContactUrl>>,
}


#[derive(Debug, Clone, Serialize)]
pub struct SendContactMessage {
    
    pub to: String,
  
    pub contacts: Vec<Contact>,
}


#[derive(Debug, Clone, Serialize)]
pub struct SendReactionMessage {
  
    pub to: String,
  
    pub message_id: String,

    pub emoji: String,
}


#[derive(Debug, Clone, Serialize)]
pub struct MarkMessageAsRead {
    
    pub message_id: String,
}


#[derive(Debug, Clone, Deserialize)]
pub struct SendMessageResponse {
  
    pub messaging_product: String,
   
    pub contacts: Vec<MessageResponseContact>,
   
    pub messages: Vec<MessageResponseMessage>,
}


#[derive(Debug, Clone, Deserialize)]
pub struct MessageResponseContact {
  
    pub input: String,
   
    pub wa_id: String,
}


#[derive(Debug, Clone, Deserialize)]
pub struct MessageResponseMessage {
   
    pub id: String,
}


#[derive(Debug, Clone, Serialize)]
pub struct SendAddressMessage {
 
    pub to: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_type: Option<String>,
  
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_text: Option<String>,
}


#[derive(Debug, Clone, Serialize)]
pub struct InteractiveCtaUrlButton {
  
    pub r#type: String,

    pub title: String,
   
    pub url: String,
}


#[derive(Debug, Clone, Serialize)]
pub struct SendInteractiveCtaUrlButtonMessage {
    
    pub to: String,
  
    pub body: String,
  
    pub buttons: Vec<InteractiveCtaUrlButton>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_text: Option<String>,
   
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_text: Option<String>,
}


#[derive(Debug, Clone, Serialize)]
pub struct InteractiveFlowParams {
   
    pub to: String,
  
    pub flow_id: String,
 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_token: Option<String>,
 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_cta: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_title: Option<String>,
   
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_description: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}


#[derive(Debug, Clone, Serialize)]
pub struct SendInteractiveLocationRequestParams {
   
    pub to: String,
   
    pub body: String,
 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_text: Option<String>,
}


#[derive(Debug, Clone, Serialize)]
pub struct MessageContext {

    pub message_id: String,
}
