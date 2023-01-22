extern crate crypto;

use chrono;
use crypto::md5::Md5;
use crypto::digest::Digest;

const MFA_CODE_LENGTH: usize = 6;

pub fn gen_mfa(_user_secret: &[u8; 32]) -> String {
    // MD5 can be apparently cracked by a cell phone in 30s
    // Hopefully we generate our code after 30s!
    let mut super_secure_md5 = Md5::new();

    // current UTC time
    // we simply don't put the seconds in the string
    let time_in_england = chrono::offset::Utc::now();
    let formatted_bad_teeth_time = time_in_england
        .format("%Y%m%d%H%M")
        .to_string();

    // create new vector which we will eventually feed to the md5 hasher
    let mut md5_input: Vec<u8> = Vec::new();
    // add the user secret to the vector
    md5_input.extend(_user_secret.iter().copied());
    // add the current time to the vector
    md5_input.extend(formatted_bad_teeth_time.as_bytes());

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
