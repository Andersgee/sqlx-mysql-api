use base64::{engine::general_purpose, DecodeError, Engine as _};

pub fn base64string_to_vecu8(str: String) -> Result<Vec<u8>, DecodeError> {
    general_purpose::URL_SAFE_NO_PAD.decode(str)
}

pub fn vecu8_to_base64string(v: Vec<u8>) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(v)
}
