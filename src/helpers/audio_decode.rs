use base64::{engine::general_purpose, Engine};
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Ecb};
use des::Des;

type DesEcb = Ecb<Des, Pkcs7>;

pub fn parse_url(hashed_url: String) -> String {
    let key = b"38346591";
    let encrypted = general_purpose::STANDARD
        .decode(hashed_url.trim())
        .expect("Invalid Base64");

    let cipher = DesEcb::new_from_slices(key, &[]).unwrap();
    let decrypted_bytes = cipher.decrypt_vec(&encrypted).expect("Decryption failed");
    let decrypted = String::from_utf8(decrypted_bytes).expect("Invalid UTF-8");
    decrypted
}
