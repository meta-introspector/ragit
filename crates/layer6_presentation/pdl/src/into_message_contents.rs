use crate::error::Error;
use crate::message::MessageContent;
use crate::image::ImageType;
use crate::unescape_pdl_tokens::unescape_pdl_tokens;
use crate::try_parse_inline_block::try_parse_inline_block;

pub fn into_message_contents(s: &str, curr_dir: &str) -> Result<Vec<MessageContent>, Error> {
    let bytes = s.as_bytes().iter().copied().collect::<Vec<_>>();
    let mut index = 0;
    let mut result = vec![];
    let mut string_buffer = vec![];

    loop {
        match bytes.get(index) {
            Some(b'<') => match try_parse_inline_block(&bytes, index, curr_dir) {
                Ok(Some((image_type, bytes, new_index))) => {
                    if !string_buffer.is_empty() {
                        match String::from_utf8(string_buffer.clone()) {
                            Ok(s) => {
                                result.push(MessageContent::String(unescape_pdl_tokens(&s)));
                            }
                            Err(e) => {
                                return Err(e.into());
                            }
                        }
                    }


                    result.push(MessageContent::Image { image_type: image_type.into(), bytes });
                    index = new_index;
                    string_buffer = vec![];
                    continue;
                }
                Ok(None) => {
                    string_buffer.push(b'<');
                }
                Err(e) => {
                    return Err(e);
                }
            },
            Some(b) => {
                string_buffer.push(*b);
            }
            None => {
                if !string_buffer.is_empty() {
                    match String::from_utf8(string_buffer) {
                        Ok(s) => {
                            result.push(MessageContent::String(unescape_pdl_tokens(&s)));
                        }
                        Err(e) => {
                            return Err(e.into());
                        }
                    }
                }

                break;
            }
        }

        index += 1;
    }

    Ok(result)
}
