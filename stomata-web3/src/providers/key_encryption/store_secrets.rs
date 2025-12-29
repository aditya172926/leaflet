use std::{fs, io, path::PathBuf};

use crate::providers::{
    encrypt_secret::{decrypt_private_key, encrypt_private_key},
    key_encryption::{
        errors::StorageError,
        structs::{EncryptPrivateKey, KeyMetadata},
    },
};

// ==== Storage Functions ====
pub fn get_storage_directory() -> Result<PathBuf, StorageError> {
    let home = dirs::home_dir().ok_or_else(|| {
        StorageError::IoError(io::Error::new(
            io::ErrorKind::NotFound,
            "Could not find home directory",
        ))
    })?;
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

/// Validate key name (no path separators, special chars, etc.)
fn validate_key_name(name: &str) -> Result<(), StorageError> {
    if name.is_empty() {
        return Err(StorageError::InvalidKeyName(
            "Key name cannot be empty".to_string(),
        ));
    }

    if name.contains('/') || name.contains('\\') || name.contains("..") {
        return Err(StorageError::InvalidKeyName(
            "Key name cannot contain path separators".to_string(),
        ));
    }

    if name.starts_with('.') {
        return Err(StorageError::InvalidKeyName(
            "Key name cannot start with a dot".to_string(),
        ));
    }

    Ok(())
}

/// Get the file path for a named key
fn get_key_path(name: &str) -> Result<PathBuf, StorageError> {
    validate_key_name(name)?;
    let keys_dir = get_keys_dir()?;
    Ok(keys_dir.join(format!("{}.json", name)))
}

/// Save an encrypted key to disk
pub fn save_encrypted_key(name: &str, encrypted: &EncryptPrivateKey) -> Result<(), StorageError> {
    init_storage()?;

    let key_path = get_key_path(name)?;

    // Check if key already exists
    if key_path.exists() {
        return Err(StorageError::KeyAlreadyExists(name.to_string()));
    }

    // Add metadata
    let encrypted_with_meta = EncryptPrivateKey {
        crypto_key: encrypted.crypto_key.clone(),
        metadata: Some(KeyMetadata {
            name: name.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        }),
    };

    let json = serde_json::to_string_pretty(&encrypted_with_meta)?;
    fs::write(&key_path, json)?;

    // Set restrictive permissions on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let permissions = fs::Permissions::from_mode(0o600);
        fs::set_permissions(&key_path, permissions)?;
    }

    Ok(())
}

/// Load an encrypted key from disk
pub fn load_encrypted_key(name: &str) -> Result<EncryptPrivateKey, StorageError> {
    let key_path = get_key_path(name)?;

    if !key_path.exists() {
        return Err(StorageError::KeyNotFound(name.to_string()));
    }

    let json = fs::read_to_string(&key_path)?;
    let encrypted: EncryptPrivateKey = serde_json::from_str(&json)?;

    Ok(encrypted)
}

/// List all stored key names
pub fn list_keys() -> Result<Vec<String>, StorageError> {
    let keys_dir = get_keys_dir()?;

    if !keys_dir.exists() {
        return Ok(Vec::new());
    }

    let mut keys = Vec::new();

    for entry in fs::read_dir(keys_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                keys.push(name.to_string());
            }
        }
    }

    keys.sort();
    Ok(keys)
}

/// Delete a stored key
pub fn delete_key(name: &str) -> Result<(), StorageError> {
    let key_path = get_key_path(name)?;

    if !key_path.exists() {
        return Err(StorageError::KeyNotFound(name.to_string()));
    }

    fs::remove_file(&key_path)?;
    Ok(())
}

/// Check if a key exists
pub fn key_exists(name: &str) -> Result<bool, StorageError> {
    let key_path = get_key_path(name)?;
    Ok(key_path.exists())
}

// === High-level convenience functions ===

/// Store a new private key with encryption
pub fn store_key(name: &str, private_key: &[u8], password: &str) -> Result<(), StorageError> {
    let encrypted = encrypt_private_key(private_key, password).ok_or_else(|| {
        StorageError::IoError(io::Error::new(io::ErrorKind::Other, "Encryption failed"))
    })?;

    save_encrypted_key(name, &encrypted)?;
    Ok(())
}

/// Retrieve and decrypt a private key
pub fn retrieve_key(name: &str, password: &str) -> Result<Vec<u8>, StorageError> {
    let encrypted = load_encrypted_key(name)?;

    decrypt_private_key(&encrypted, password).ok_or_else(|| {
        StorageError::IoError(io::Error::new(
            io::ErrorKind::InvalidData,
            "Decryption failed - wrong password?",
        ))
    })
}
