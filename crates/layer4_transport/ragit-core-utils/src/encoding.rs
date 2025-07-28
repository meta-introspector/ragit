use base64::engine::general_purpose;
use base64::Engine;

pub fn encode_base64(bytes: &[u8]) -> String {
    general_purpose::STANDARD.encode(bytes)
}
