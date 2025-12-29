use std::io;


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