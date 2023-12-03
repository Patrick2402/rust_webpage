use base64::{engine::general_purpose, Engine as _};
use rand::Rng;

pub fn decode_salt(salt: &str) -> Option<[u8; 16]> {
    general_purpose::STANDARD_NO_PAD.decode(salt).ok()?[0..16]
        .try_into()
        .ok()
}

pub fn encode_salt(salt: [u8; 16]) -> String {
    general_purpose::STANDARD_NO_PAD.encode(salt)
}

pub fn generate_salt() -> [u8; 16] {
    rand::thread_rng().gen()
}
