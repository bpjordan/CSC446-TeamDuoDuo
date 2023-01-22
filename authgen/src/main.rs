use mfalib;
use base64::{Engine as _, engine::general_purpose};

static _SAMPLE_BASE64_USER_SECRET: &str = "gzBsRiEnc3Kwc/26S3gklyz5M4UUOztqbO4pbhtgDi4=";

fn main() {
    let user_secret = &general_purpose::STANDARD
    .decode(_SAMPLE_BASE64_USER_SECRET)
    .unwrap();

    let mfa_code = mfalib::gen_mfa(
        user_secret
        // &Vec<u8> -> [u8]
        .as_slice()
        // [u8] -> [u8; 32]
        .try_into()
        // I promise this won't fail
        .unwrap()
    );

    println!("Code: {}", mfa_code);
}
