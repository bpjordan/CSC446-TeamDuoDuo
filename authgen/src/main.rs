use mfalib;

static _SAMPLE_BASE64_USER_SECRET: &str = "gzBsRiEnc3Kwc/26S3gklyz5M4UUOztqbO4pbhtgDi4=";

fn main() {
    println!("Code: {}", mfalib::gen_mfa(
            // cap length to 32 bytes
            _SAMPLE_BASE64_USER_SECRET[..32]
            // &str -> [u8]
            .as_bytes()
            // [u8] -> [u8; 32]
            .try_into()
            // I promise this won't fail
            .unwrap()
    ));
}
