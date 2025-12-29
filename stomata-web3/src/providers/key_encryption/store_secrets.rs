use std::{fs, io, path::PathBuf};

use crate::providers::key_encryption::errors::StorageError;

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