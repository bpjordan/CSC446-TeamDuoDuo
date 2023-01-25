use mfalib;
use std::env;
use base64::{Engine as _, engine::general_purpose};
use std::time::{SystemTime, UNIX_EPOCH};

static _SAMPLE_BASE64_USER_SECRET: &str = "gzBsRiEnc3Kwc/26S3gklyz5M4UUOztqbO4pbhtgDi4=";

fn main() {
    let user_secret = &general_purpose::STANDARD
    .decode(_SAMPLE_BASE64_USER_SECRET)
    .unwrap();

    let mfa_code = mfalib::gen_mfa(
        user_secret
        // &Vec<u8> -> [u8]
        .as_slice()[..32]
        // [u8] -> [u8; 32]
        // we can guarantee this will work thanks to the previous [..32]
        .try_into()
        // I promise this won't fail
        .unwrap()
    );

    // Read command-line arguments
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 && (args[1] == "-q" || args[1] == "--quiet") {
        println!("{}",mfa_code);
        return;
    }

    // number of seconds since January 1, 1970
    let current_unix_time = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Failed to get current time. Is your system time set to before 1/1/1970?")
    .as_secs();

    let time_left = 30 - (current_unix_time % mfalib::CODE_TIME);

    println!("Code: {}", mfa_code);
    println!("This code will last for the next {} seconds.", time_left);
}
