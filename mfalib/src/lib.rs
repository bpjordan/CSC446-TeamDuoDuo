extern crate crypto;

use crypto::{md5::Md5, digest::Digest};
use std::time::{SystemTime, UNIX_EPOCH};

const MFA_CODE_LENGTH: usize = 6;
pub const CODE_TIME: u64 = 30; // seconds

pub fn gen_mfa(_user_secret: &[u8; 32]) -> String {
    // MD5 can be apparently cracked by a cell phone in 30s
    // Hopefully we generate our code after 30s!
    let mut super_secure_md5 = Md5::new();

    // number of seconds since January 1, 1970: u64
    let mut current_unix_time = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Failed to get current time. Is your system time set to before 1/1/1970?")
    .as_secs();

    // round to nearest CODE_TIME seconds
    current_unix_time -= current_unix_time % CODE_TIME;

    // create new vector which we will eventually feed to the md5 hasher
    let mut md5_input: Vec<u8> = Vec::new();
    // add the user secret to the vector
    md5_input.extend(_user_secret.iter().copied());
    // add the current time to the vector
    md5_input.extend(current_unix_time.to_string().as_bytes());

    // hash string
    super_secure_md5.input(&md5_input);

    // collect the digits from the hash
    let mut hash_only_numbers = String::new();
    for hash_char in super_secure_md5.result_str().chars() {
        if hash_char.is_numeric() {
            hash_only_numbers.push(hash_char);
        }
    }

    // return the first MFA_CODE_LENGTH characters
    return String::from(&hash_only_numbers[..MFA_CODE_LENGTH]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        gen_mfa(&[b'0'; 32]);
    }
}
