
use ragit_types::pdl_error::PdlError as Error;
use ragit_types::pdl_types::ImageType;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use ragit_fs::{extension, join, read_bytes};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;

lazy_static! {
    static ref MEDIA_RE: Regex = Regex::new(r"^media\((.+)\)$").unwrap();
    static ref RAW_MEDIA_RE: Regex = Regex::new(r"^raw_media\(([a-zA-Z0-9]+):([^:]+)\)$").unwrap();
}

pub fn try_parse_inline_block(
    bytes: &[u8],
    index: usize,
    curr_dir: &str,
) -> Result<Option<(ImageType, Vec<u8>, usize)>, Error> {
    match super::try_get_pdl_token::try_get_pdl_token(bytes, index) {
        Some((token, new_index)) => {
            let media_re = &MEDIA_RE;
            let raw_media_re = &RAW_MEDIA_RE;

            if let Some(cap) = raw_media_re.captures(token) {
                let image_type = String::from_utf8_lossy(&cap[1]).to_string();
                let image_bytes = String::from_utf8_lossy(&cap[2]).to_string();

                Ok(Some((
                    ImageType::from_extension(&image_type)?,
                    BASE64_STANDARD.decode(image_bytes).map_err(Error::Base64)?,
                    new_index,
                )))
            } else if let Some(cap) = media_re.captures(token) {
                let path = &cap[1];
                let file = join(curr_dir, &String::from_utf8_lossy(path).to_string())?;
                Ok(Some((
                    ImageType::from_extension(&extension(&file)?.unwrap_or(String::new())).map_err(Error::ImageType)?,
                    read_bytes(&file)?,
                    new_index,
                )))
            } else {
                Err(Error::InvalidInlineBlock)
            }
        }

        None => Ok(None),
    }
}
