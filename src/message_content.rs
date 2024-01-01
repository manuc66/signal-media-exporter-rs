use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageContent {
    pub timestamp: Option<i64>,
    pub attachments: Option<Vec<Attachment>>,
    pub source: Option<String>,
    pub source_device: Option<i32>,
    #[serde(rename = "sent_at")]
    pub sent_at: Option<i64>,
    #[serde(rename = "received_at")]
    pub received_at: i64,
    pub conversation_id: Option<String>,
    pub unidentified_delivery_received: Option<bool>,
    pub type_: Option<String>,
    pub schema_version: i32,
    pub id: String,
    pub body: Option<String>,
    pub contact: Option<Vec<serde_json::Value>>,
    #[serde(rename = "decrypted_at")]
    pub decrypted_at: Option<i64>,
    pub errors: Option<Vec<serde_json::Value>>,
    pub flags: Option<i32>,
    pub has_attachments: Option<i32>,
    pub has_visual_media_attachments: Option<i32>,
    pub is_view_once: Option<bool>,
    pub preview: Option<Vec<serde_json::Value>>,
    pub required_protocol_version: Option<i32>,
    pub supported_version_at_receive: Option<i32>,
    pub quote: Option<serde_json::Value>,
    pub sticker: Option<serde_json::Value>,
    pub read_status: i32,
    pub seen_status: i32,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    #[serde(rename = "attachment_identifier")]
    pub attachment_identifier: Option<String>,
    pub content_type: String,
    #[serde(rename = "file_name")]
    pub file_name: Option<String>,
    #[serde(rename = "path")]
    pub path: Option<String>
    // ... include other fields as necessary
}
