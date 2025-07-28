use serde::{Deserialize, Serialize};
use std::fmt;
use base64::Engine;
use crate::chunk::atomic_token::AtomicToken;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageContent {
    String(String),
    Image { image_type: crate::image::image_struct::ImageType, bytes: Vec<u8> },
}

impl MessageContent {
    pub fn is_string(&self) -> bool {
        matches!(self, MessageContent::String(_))
    }

    pub fn unwrap_str(&self) -> &str {
        match self {
            MessageContent::String(s) => s,
            _ => panic!("not a string"),
        }
    }
}

impl fmt::Display for MessageContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageContent::String(s) => write!(f, "{}", s),
            MessageContent::Image { image_type, bytes } => {
                write!(f, "<|raw_media({}:{})|>", image_type.to_extension(), base64::prelude::BASE64_STANDARD.encode(bytes))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Role {
    User,
    Assistant,
    System,
    Reasoning,
}

impl Role {
    pub fn to_api_string(&self, is_google: bool) -> &str {
        match self {
            Role::User => "user",
            Role::Assistant => "assistant",
            Role::System => {
                if is_google {
                    "user"
                } else {
                    "system"
                }
            }
            Role::Reasoning => "user",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum PdlRole {
    User,
    Assistant,
    System,
    Reasoning,
    Schema,
}

impl From<&str> for PdlRole {
    fn from(s: &str) -> Self {
        match s {
            "user" => PdlRole::User,
            "assistant" => PdlRole::Assistant,
            "system" => PdlRole::System,
            "reasoning" => PdlRole::Reasoning,
            "schema" => PdlRole::Schema,
            _ => panic!("Invalid PdlRole"),
        }
    }
}

impl From<PdlRole> for Role {
    fn from(pdl_role: PdlRole) -> Self {
        match pdl_role {
            PdlRole::User => Role::User,
            PdlRole::Assistant => Role::Assistant,
            PdlRole::System => Role::System,
            PdlRole::Reasoning => Role::Reasoning,
            PdlRole::Schema => panic!("Schema role cannot be converted to Role"),
        }
    }
}

impl From<AtomicToken> for MessageContent {
    fn from(token: AtomicToken) -> Self {
        match token {
            AtomicToken::String { data, .. } => MessageContent::String(data),
            AtomicToken::Image(image) => MessageContent::Image {
                image_type: image.image_type,
                bytes: image.bytes,
            },
            AtomicToken::WebImage { subst, .. } => MessageContent::String(subst),
            AtomicToken::PageBreak => MessageContent::String(String::new()),
            AtomicToken::ChunkExtraInfo(_) => MessageContent::String(String::new()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: Vec<MessageContent>,
}

impl Message {
    pub fn simple_message(role: Role, content: String) -> Self {
        Self {
            role,
            content: vec![MessageContent::String(content)],
        }
    }

    pub fn has_image(&self) -> bool {
        self.content.iter().any(|c| matches!(c, MessageContent::Image { .. }))
    }

    pub fn is_valid_system_prompt(&self) -> bool {
        self.role == Role::System && self.content.len() == 1 && self.content[0].is_string()
    }

    pub fn is_user_prompt(&self) -> bool {
        self.role == Role::User && self.content.len() == 1 && self.content[0].is_string()
    }

    pub fn is_assistant_prompt(&self) -> bool {
        self.role == Role::Assistant && self.content.len() == 1 && self.content[0].is_string()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ImageType {
    Png,
    Jpeg,
    Gif,
    Webp,
    Svg,
    Bmp,
}

impl ImageType {
    pub fn from_extension(s: &str) -> Result<Self, String> {
        match s.to_ascii_lowercase().as_str() {
            "png" => Ok(ImageType::Png),
            "jpg" | "jpeg" => Ok(ImageType::Jpeg),
            "gif" => Ok(ImageType::Gif),
            "webp" => Ok(ImageType::Webp),
            "svg" => Ok(ImageType::Svg),
            "bmp" => Ok(ImageType::Bmp),
            _ => Err(format!("Unsupported image type: {s}")),
        }
    }

    pub fn to_extension(&self) -> &str {
        match self {
            ImageType::Png => "png",
            ImageType::Jpeg => "jpeg",
            ImageType::Gif => "gif",
            ImageType::Webp => "webp",
            ImageType::Svg => "svg",
            ImageType::Bmp => "bmp",
        }
    }

    pub fn get_media_type(&self) -> &str {
        match self {
            ImageType::Png => "image/png",
            ImageType::Jpeg => "image/jpeg",
            ImageType::Gif => "image/gif",
            ImageType::Webp => "image/webp",
            ImageType::Svg => "image/svg+xml",
            ImageType::Bmp => "image/bmp",
        }
    }
}
