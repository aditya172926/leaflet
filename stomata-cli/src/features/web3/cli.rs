use clap::{Parser, Subcommand, ValueEnum};

/// Web3 developer tools CLI
///
/// A command-line interface providing various utilities for Web3 development,
/// including address validation and secure key management.
///
/// # Examples
///
/// ```bash
/// # Validate an Ethereum address
/// stomata web3 av -a 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb
///
/// # Encrypt a key
/// stomata web3 key encrypt -n my-secret-key
///
/// # List all stored keys
/// stomata web3 key list
/// ```
#[derive(Parser, Clone)]
#[command(name = "web3")]
#[command(about = "Web3 developer tools")]
pub struct Web3Cli {
    /// The web3 tool to execute
    #[command(subcommand)]
    pub tool: Web3Tool,
}

/// Available Web3 tools
///
/// Top-level commands for the Web3 CLI, each providing specific
/// functionality for blockchain development and key management.
#[derive(Subcommand, Clone)]
pub enum Web3Tool {
    /// Validates Ethereum addresses for correctness
    ///
    /// Checks if the provided address follows the Ethereum address format
    /// and validates the checksum if present.
    ///
    /// # Examples
    ///
    /// ```bash
    /// stomata web3 av -a 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb
    /// stomata web3 av -a 0xinvalid  # using alias
    /// ```
    #[command(name = "address-validator", alias = "av")]
    AddressValidator {
        /// Ethereum address to validate
        ///
        /// Should be a 42-character string starting with "0x" followed by
        /// 40 hexadecimal characters. The checksum will be validated if present.
        #[arg(short, long, required = true)]
        address: String,
    },
    /// Key management operations
    ///
    /// Securely store, retrieve, and manage cryptographic keys and secrets.
    #[command(subcommand)]
    Key(KeySubCommands),
}

/// Output format for decrypted data
///
/// Determines how decrypted key data should be displayed to the user.
#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    /// Display output as hexadecimal string
    ///
    /// Useful for binary data or when the exact byte representation is needed.
    Hex,

    /// Display output as UTF-8 encoded string
    ///
    /// Default format, suitable for text-based secrets like passwords or API keys.
    Utf8,
}

/// Key management subcommands
///
/// Operations for securely storing and retrieving encrypted keys.
/// All keys are stored with encryption at rest.
#[derive(Subcommand, Clone)]
pub enum KeySubCommands {
    /// Encrypt and store a new key
    ///
    /// Prompts for the key value (input is hidden) and password (hidden), encrypts it,
    /// and stores it under the specified name.
    ///
    /// # Examples
    ///
    /// ```bash
    /// stomata web3 key encrypt -n my-api-key
    /// stomata web3 key e -n wallet-seed  # using alias
    /// ```
    #[command(name = "encrypt", alias = "e")]
    Encrypt {
        /// Name identifier for the key
        ///
        /// This name will be used to retrieve the key later.
        /// Must be unique among stored keys.
        #[arg(short, long, required = true)]
        name: String,
    },

    /// Decrypt and display a stored key
    ///
    /// Retrieves the encrypted key by name, decrypts it,
    /// and displays it in the requested format.
    ///
    /// # Examples
    ///
    /// ```bash
    /// stomata web3 key decrypt -n my-api-key
    /// stomata web3 key d -n wallet-seed --format hex  # using alias
    /// ```
    #[command(name = "decrypt", alias = "d")]
    Decrypt {
        /// Name of the key to decrypt
        ///
        /// Must match the name used when the key was encrypted.
        #[arg(short, long, required = true)]
        name: String,

        /// Output format for the decrypted data
        ///
        /// Choose 'hex' for binary data or 'utf8' (default) for text.
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Utf8)]
        format: OutputFormat,
    },

    /// List all stored key names
    ///
    /// Displays the names of all encrypted keys currently stored.
    /// Does not decrypt or display key values.
    ///
    /// # Examples
    ///
    /// ```bash
    /// stomata web3 key list
    /// stomata web3 key l  # using alias
    /// ```
    #[command(name = "list", alias = "l")]
    List {},

    /// Delete a stored key permanently
    ///
    /// Removes the encrypted key from storage. This action cannot be undone.
    ///
    /// # Examples
    ///
    /// ```bash
    /// stomata web3 key delete -n old-api-key
    /// stomata web3 key del -n unused-key  # using alias
    /// ```
    #[command(name = "delete", alias = "del")]
    Delete {
        /// Name of the key to delete
        ///
        /// The key will be permanently removed from storage.
        #[arg(short, long, required = true)]
        name: String,
    },
}
