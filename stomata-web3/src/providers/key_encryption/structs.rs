use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptPrivateKey {
    pub crypto_key: CryptoData,
    pub metadata: Option<KeyMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyMetadata {
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoData {
    pub cipher: String,
    pub salt: String,
    pub nonce: String,
    pub ciphertext: String,
}
