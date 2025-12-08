use stomata_web3::providers::address::{AddressValidator, ValidationResult};

pub fn validate_address(address: &str) {
    let result = AddressValidator::validate(address);
    println!("{:?}", result);
}
