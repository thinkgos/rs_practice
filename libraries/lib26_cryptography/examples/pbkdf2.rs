use data_encoding::HEXUPPER;
use std::num::NonZeroU32;

use ring::{
    digest, pbkdf2,
    rand::{self, SecureRandom},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = "Guess Me If You Can111!";
    let wrong_secret = "Definitely not the correct password";

    // 盐
    let mut salt = [0u8; digest::SHA512_OUTPUT_LEN];
    // 迭代次数
    let n_iter = NonZeroU32::new(100_000).unwrap();
    // 生成随机盐
    rand::SystemRandom::new().fill(&mut salt).unwrap();

    let mut pbkdf2_hash = [0u8; digest::SHA512_OUTPUT_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        secret.as_bytes(),
        &mut pbkdf2_hash,
    );
    println!("<------------------ 加签 ------------------------>");
    println!("Salt: {}", HEXUPPER.encode(&salt));
    println!("PBKDF2 hash: {}", HEXUPPER.encode(&pbkdf2_hash));

    println!("<------------------ 验签 ------------------------>");
    let should_succeed = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        secret.as_bytes(),
        &pbkdf2_hash,
    );
    assert!(should_succeed.is_ok());

    let should_fail = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        wrong_secret.as_bytes(),
        &pbkdf2_hash,
    );
    assert!(!should_fail.is_ok());

    Ok(())
}
