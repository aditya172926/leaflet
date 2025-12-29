use std::{fs, io, path::PathBuf};

use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
use argon2::Argon2;
use rand::random;

#[derive(Debug)]
pub struct EncryptPrivateKey {
    pub crypto_key: CryptoData,
    pub metadata: Option<KeyMetadata>
}

#[derive(Debug)]
pub struct KeyMetadata {
    pub name: String,
    pub created_at: String
}

#[derive(Debug)]
pub struct CryptoData {
    pub cipher: String,
    pub salt: String,
    pub nonce: String,
    pub ciphertext: String,
}

#[derive(Debug)]
pub enum StorageError {
    IoError(io::Error),
    SerdeError(serde_json::Error),
    KeyNotFound(String),
    KeyAlreadyExists(String),
    InvalidKeyName(String),
}

impl From<io::Error> for StorageError {
    fn from(err: io::Error) -> Self {
        StorageError::IoError(err)
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(err: serde_json::Error) -> Self {
        StorageError::SerdeError(err)
    }
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StorageError::IoError(e) => write!(f, "IO error: {}", e),
            StorageError::SerdeError(e) => write!(f, "Serialization error: {}", e),
            StorageError::KeyNotFound(name) => write!(f, "Key '{}' not found", name),
            StorageError::KeyAlreadyExists(name) => write!(f, "Key '{}' already exists", name),
            StorageError::InvalidKeyName(name) => write!(f, "Invalid key name: '{}'", name),
        }
    }
}

impl std::error::Error for StorageError {}

// ==== Core Encryption Functions ====

fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .unwrap();
    key
}

pub fn encrypt_private_key(pk: &[u8], password: &str) -> Option<EncryptPrivateKey> {
    let salt = random::<[u8; 16]>();
    let nonce = random::<[u8; 12]>();
    let key = derive_key(password, &salt);
    let cipher = match Aes256Gcm::new_from_slice(&key) {
        Ok(res) => res,
        Err(_err) => {
            return None;
        }
    };
    let ciphertext = match cipher.encrypt(Nonce::from_slice(&nonce), pk) {
        Ok(c_text) => c_text,
        Err(err) => {
            eprintln!("Error in encrypting private key {:?}", err);
            return None;
        }
    };

    Some(EncryptPrivateKey {
        crypto_key: CryptoData {
            cipher: "aes-256-gcm".to_string(),
            salt: hex::encode(salt),
            nonce: hex::encode(nonce),
            ciphertext: hex::encode(ciphertext),
        },
        metadata: None
    })
}

pub fn decrypt_private_key(data: &EncryptPrivateKey, password: &str) -> Option<Vec<u8>> {
    let salt = match hex::decode(&data.crypto_key.salt) {
        Ok(salt) => salt,
        Err(err) => {
            eprintln!("Error in decoding salt {:?}", err);
            return None;
        }
    };

    let nonce = match hex::decode(&data.crypto_key.nonce) {
        Ok(nonce) => nonce,
        Err(err) => {
            eprintln!("Error in decoding nonce {:?}", err);
            return None;
        }
    };

    let ciphertext = match hex::decode(&data.crypto_key.ciphertext) {
        Ok(ciphertext) => ciphertext,
        Err(err) => {
            eprintln!("Error in decoding ciphertext {:?}", err);
            return None;
        }
    };

    let key = derive_key(password, &salt);
    let cipher = match Aes256Gcm::new_from_slice(&key) {
        Ok(cipher) => cipher,
        Err(err) => {
            eprintln!("Error in generating cipher {:?}", err);
            return None;
        }
    };

    match cipher.decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref()) {
        Ok(res) => Some(res),
        Err(err) => {
            eprintln!("Error in decrypting key {:?}", err);
            return None;
        }
    }
}

// ==== Storage Functions ====
pub fn get_storage_directory() -> Result<PathBuf, StorageError> {
    let home = dirs::home_dir()
        .ok_or_else(|| StorageError::IoError(
            io::Error::new(io::ErrorKind::NotFound, "Could not find home directory")
        ))?;
    let storage_dir = home.join(".stomataKeys");
    Ok(storage_dir)
}

/// Get the directory where encrypted keys are stored
pub fn get_keys_dir() -> Result<PathBuf, StorageError> {
    let storage_dir = get_storage_directory()?;
    let keys_dir = storage_dir.join("keys");
    Ok(keys_dir)
}

/// Create the storage dirs if they don't exist
pub fn init_storage() -> Result<(), StorageError> {
    let keys_dir = get_keys_dir()?;
    
    if !keys_dir.exists() {
        fs::create_dir_all(&keys_dir)?;
        
        // Set restrictive permissions on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = fs::Permissions::from_mode(0o700);
            fs::set_permissions(&keys_dir, permissions)?;
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_basic() {
        let private_key = b"my_super_secret_private_key_1234";
        let password = "strong_password_123";

        let encrypted =
            encrypt_private_key(private_key, password).expect("Encryption should succeed");

        let decrypted =
            decrypt_private_key(&encrypted, password).expect("Decryption should succeed");

        assert_eq!(private_key.as_slice(), decrypted.as_slice());
    }

    #[test]
    fn test_wrong_password_fails() {
        let private_key = b"my_super_secret_private_key";
        let password = "correct_password";
        let wrong_password = "wrong_password";

        let encrypted =
            encrypt_private_key(private_key, password).expect("Encryption should succeed");

        let decrypted = decrypt_private_key(&encrypted, wrong_password);

        assert!(
            decrypted.is_none(),
            "Decryption with wrong password should fail"
        );
    }

    #[test]
    fn test_empty_private_key() {
        let private_key = b"";
        let password = "password";

        let encrypted = encrypt_private_key(private_key, password)
            .expect("Encryption of empty data should succeed");

        let decrypted =
            decrypt_private_key(&encrypted, password).expect("Decryption should succeed");

        assert_eq!(private_key.as_slice(), decrypted.as_slice());
    }

    #[test]
    fn test_long_private_key() {
        let private_key = vec![0u8; 10000];
        let password = "password";

        let encrypted =
            encrypt_private_key(&private_key, password).expect("Encryption should succeed");

        let decrypted =
            decrypt_private_key(&encrypted, password).expect("Decryption should succeed");

        assert_eq!(private_key, decrypted);
    }

    #[test]
    fn test_special_characters_in_password() {
        let private_key = b"secret_key";
        let password = "p@ssw0rd!#$%^&*()_+-=[]{}|;:,.<>?/~`";

        let encrypted =
            encrypt_private_key(private_key, password).expect("Encryption should succeed");

        let decrypted =
            decrypt_private_key(&encrypted, password).expect("Decryption should succeed");

        assert_eq!(private_key.as_slice(), decrypted.as_slice());
    }

    #[test]
    fn test_different_encryptions_produce_different_ciphertexts() {
        let private_key = b"same_key";
        let password = "same_password";

        let encrypted1 =
            encrypt_private_key(private_key, password).expect("First encryption should succeed");
        let encrypted2 =
            encrypt_private_key(private_key, password).expect("Second encryption should succeed");

        // Salt and nonce should be different
        assert_ne!(encrypted1.crypto_key.salt, encrypted2.crypto_key.salt);
        assert_ne!(encrypted1.crypto_key.nonce, encrypted2.crypto_key.nonce);
        assert_ne!(
            encrypted1.crypto_key.ciphertext,
            encrypted2.crypto_key.ciphertext
        );

        // But both should decrypt to the same plaintext
        let decrypted1 = decrypt_private_key(&encrypted1, password).unwrap();
        let decrypted2 = decrypt_private_key(&encrypted2, password).unwrap();
        assert_eq!(decrypted1, decrypted2);
        assert_eq!(private_key.as_slice(), decrypted1.as_slice());
    }

    #[test]
    fn test_corrupted_salt_fails() {
        let private_key = b"secret_key";
        let password = "password";

        let mut encrypted =
            encrypt_private_key(private_key, password).expect("Encryption should succeed");

        // Corrupt the salt
        encrypted.crypto_key.salt = "invalid_hex_string".to_string();

        let decrypted = decrypt_private_key(&encrypted, password);
        assert!(
            decrypted.is_none(),
            "Decryption with corrupted salt should fail"
        );
    }

    #[test]
    fn test_corrupted_nonce_fails() {
        let private_key = b"secret_key";
        let password = "password";

        let mut encrypted =
            encrypt_private_key(private_key, password).expect("Encryption should succeed");

        // Corrupt the nonce
        encrypted.crypto_key.nonce = "not_valid_hex".to_string();

        let decrypted = decrypt_private_key(&encrypted, password);
        assert!(
            decrypted.is_none(),
            "Decryption with corrupted nonce should fail"
        );
    }

    #[test]
    fn test_corrupted_ciphertext_fails() {
        let private_key = b"secret_key";
        let password = "password";

        let mut encrypted =
            encrypt_private_key(private_key, password).expect("Encryption should succeed");

        // Corrupt the ciphertext by flipping a bit
        let mut bytes = hex::decode(&encrypted.crypto_key.ciphertext).unwrap();
        if !bytes.is_empty() {
            bytes[0] ^= 0xFF;
        }
        encrypted.crypto_key.ciphertext = hex::encode(bytes);

        let decrypted = decrypt_private_key(&encrypted, password);
        assert!(
            decrypted.is_none(),
            "Decryption with corrupted ciphertext should fail"
        );
    }

    #[test]
    fn test_binary_private_key() {
        let private_key: Vec<u8> = (0..=255).collect();
        let password = "password";

        let encrypted =
            encrypt_private_key(&private_key, password).expect("Encryption should succeed");

        let decrypted =
            decrypt_private_key(&encrypted, password).expect("Decryption should succeed");

        assert_eq!(private_key, decrypted);
    }

    #[test]
    fn test_hex_encoding_format() {
        let private_key = b"test_key";
        let password = "password";

        let encrypted =
            encrypt_private_key(private_key, password).expect("Encryption should succeed");

        // Verify all fields are valid hex strings
        assert!(hex::decode(&encrypted.crypto_key.salt).is_ok());
        assert!(hex::decode(&encrypted.crypto_key.nonce).is_ok());
        assert!(hex::decode(&encrypted.crypto_key.ciphertext).is_ok());

        // Verify expected lengths
        let salt_bytes = hex::decode(&encrypted.crypto_key.salt).unwrap();
        let nonce_bytes = hex::decode(&encrypted.crypto_key.nonce).unwrap();

        assert_eq!(salt_bytes.len(), 16, "Salt should be 16 bytes");
        assert_eq!(nonce_bytes.len(), 12, "Nonce should be 12 bytes");
    }
}
