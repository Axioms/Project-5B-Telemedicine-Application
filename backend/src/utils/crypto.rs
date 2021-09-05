use std::num::NonZeroU32;

use ring::{digest, pbkdf2};

static DIGEST_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const OUTPUT_LEN: usize = digest::SHA256_OUTPUT_LEN;

// password

pub fn hash_password(secret: &[u8], salt: &[u8], iterations: u32) -> Vec<u8> {
    let mut hash = vec![0u8; OUTPUT_LEN];

    let iterations = NonZeroU32::new(iterations).expect("Iterations can't be zero");

    pbkdf2::derive(DIGEST_ALG, iterations, salt, secret, &mut hash);

    hash
}

pub fn validate_password(secret: &[u8], salt: &[u8], previous: &[u8], iterations: u32) -> bool {
    let iterations = NonZeroU32::new(iterations).expect("Iterations can't be zero");
    pbkdf2::verify(DIGEST_ALG, iterations, salt, secret, previous).is_ok()
}

// Random Numbers

pub fn get_random_64() -> Vec<u8> {
    get_random(vec![0u8; 64])
}

pub fn get_random(mut array: Vec<u8>) -> Vec<u8> {
    use ring::rand::{SecureRandom, SystemRandom};
    
    SystemRandom::new()
        .fill(&mut array)
        .expect("Error generating random values");
    array
}