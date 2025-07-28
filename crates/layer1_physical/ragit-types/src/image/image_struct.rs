use crate::Uid;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

//TODO: remove this placeholder
#[derive(Debug, Error)]
pub enum Error{
    #[error("Invalid image type: {0}")]
    InvalidImageType(String),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum ImageType {
    Jpeg,
    Png,
    Gif,
    Webp,
    Svg,
    Bmp,
}

impl ImageType {
    // for anthropic api
    pub fn get_media_type(&self) -> &str {
        match self {
            ImageType::Jpeg => "image/jpeg",
            ImageType::Png => "image/png",
            ImageType::Gif => "image/gif",
            ImageType::Webp => "image/webp",

            // I'm not sure whether it'd work with anthropic api
            ImageType::Svg => "image/svg+xml",
            ImageType::Bmp => "image/bmp",
        }
    }

    pub fn from_media_type(s: &str) -> Result<Self, Error> {
        match s.to_ascii_lowercase() {
            s if s == "image/jpeg" || s == "image/jpg" => Ok(ImageType::Jpeg),
            s if s == "image/png" => Ok(ImageType::Png),
            s if s == "image/gif" => Ok(ImageType::Gif),
            s if s == "image/webp" => Ok(ImageType::Webp),
            s if s == "image/svg+xml" => Ok(ImageType::Svg),
            _ => Err(Error::InvalidImageType(s.to_string())),
        }
    }

    pub fn from_extension(ext: &str) -> Result<Self, Error> {
        match ext.to_ascii_lowercase() {
            ext if ext == "png" => Ok(ImageType::Png),
            ext if ext == "jpeg" || ext == "jpg" => Ok(ImageType::Jpeg),
            ext if ext == "gif" => Ok(ImageType::Gif),
            ext if ext == "webp" => Ok(ImageType::Webp),
            ext if ext == "svg" => Ok(ImageType::Svg),
            _ => Err(Error::InvalidImageType(ext.to_string())),
        }
    }

    pub fn infer_from_path(path: &str) -> Result<Self, Error> {
        let ext_re = Regex::new(r".+\.([^.]+)$").unwrap();

        if let Some(ext) = ext_re.captures(path) {
            ImageType::from_extension(ext.get(1).unwrap().as_str())
        } else {
            Err(Error::InvalidImageType(path.to_string()))
        }
    }

    pub fn to_extension(&self) -> &str {
        match self {
            ImageType::Jpeg => "jpg",
            ImageType::Png => "png",
            ImageType::Gif => "gif",
            ImageType::Webp => "webp",
            ImageType::Svg => "svg",
            ImageType::Bmp => "bmp",
        }
    }
}

impl TryFrom<ImageType> for image::ImageFormat {
    type Error = Error;

    fn try_from(image_type: ImageType) -> Result<Self, Error> {
        match image_type {
            ImageType::Jpeg => Ok(image::ImageFormat::Jpeg),
            ImageType::Png => Ok(image::ImageFormat::Png),
            ImageType::Gif => Ok(image::ImageFormat::Gif),
            ImageType::Webp => Ok(image::ImageFormat::WebP),
            ImageType::Svg => Err(Error::InvalidImageType(
                image_type.to_extension().to_string(),
            )),
            ImageType::Bmp => Err(Error::InvalidImageType(
                image_type.to_extension().to_string(),
            )),
        }
    }
}

impl fmt::Display for ImageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<crate::pdl_types::ImageType> for ImageType {
    fn from(pdl_image_type: crate::pdl_types::ImageType) -> Self {
        match pdl_image_type {
            crate::pdl_types::ImageType::Png => ImageType::Png,
            crate::pdl_types::ImageType::Jpeg => ImageType::Jpeg,
            crate::pdl_types::ImageType::Gif => ImageType::Gif,
            crate::pdl_types::ImageType::Webp => ImageType::Webp,
            crate::pdl_types::ImageType::Svg => ImageType::Svg,
            crate::pdl_types::ImageType::Bmp => ImageType::Bmp,
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub uid: Uid,
    pub image_type: ImageType,
    pub bytes: Vec<u8>,
}

impl fmt::Debug for Image {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.debug_struct("Image")
            .field("uid", &self.uid)
            .field("image_type", &self.image_type)
            .finish()
    }
}
