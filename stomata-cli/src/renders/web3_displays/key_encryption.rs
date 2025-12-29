use std::process::exit;

use stomata_web3::providers::store_key;

pub fn encrypt_key(name: String, pk: String) {
    let password = match rpassword::prompt_password("Add Password: ") {
        Ok(pw) => pw,
        Err(err) => {
            eprintln!("Error in reading password");
            exit(0)
        }
    };

    let res = store_key(name.as_str(), pk.as_bytes(), password.as_str());
    if let Err(err) = res {
        eprintln!("Error in encrypting key {:?}", err);
    }
}
