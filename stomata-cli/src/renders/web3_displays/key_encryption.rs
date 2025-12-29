use std::process::exit;

use stomata_web3::providers::{retrieve_key, store_key};

use crate::features::web3::cli::OutputFormat;

fn ask_sensitive_info(ask_text: &str) -> String {
    let info = match rpassword::prompt_password(ask_text) {
        Ok(pw) => pw,
        Err(err) => {
            eprintln!("Error in reading entered data");
            exit(0)
        }
    };
    info
}

pub fn encrypt_key(name: String) {
    let password = ask_sensitive_info("Password: ");
    let pk = ask_sensitive_info("Key to encrypt: ");
    let res = store_key(name.as_str(), pk.as_bytes(), password.as_str());
    if let Err(err) = res {
        eprintln!("Error in encrypting key {:?}", err);
    }
}

pub fn decrypt_key(name: String, format: OutputFormat) {
    let password = ask_sensitive_info("Password: ");
    let res = retrieve_key(name.as_str(), password.as_str());
    if let Ok(data) = res {
        // println!("{:?}", hex::encode(&data));
        match format {
            OutputFormat::Hex => println!("{:?}", hex::encode(&data)),
            OutputFormat::Utf8 => println!(
                "{:?}",
                String::from_utf8(data).expect("Failed to decrypt key to utf-8")
            ),
        }
    };
}
