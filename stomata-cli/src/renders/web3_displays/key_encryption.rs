use std::process::exit;

use stomata_web3::providers::encrypt_secret::{decrypt_private_key, encrypt_private_key};

pub fn encrypt_key(pk: String) {
    let password = match rpassword::prompt_password("Your Password") {
        Ok(pw) => pw,
        Err(err) => {
            eprintln!("Error in reading password");
            exit(0)
        }
    };

    let res = encrypt_private_key(pk.as_bytes(), password.as_str());
    if let Some(data) = res {
        println!("{:?}", data);
    }
}
