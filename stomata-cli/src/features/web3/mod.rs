//! Web3 Developer Tools
//!
//! # Features
//!
//! - **Address Validation**: Validate Ethereum addresses with checksum verification
//! - **Key Management**: Securely encrypt, decrypt, and manage cryptographic keys
//!
//! # Usage
//!
//! This crate is primarily designed to be used as a CLI application:
//!
//! ```bash
//! # Validate an Ethereum address
//! stomata web3 address-validator -a 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb
//!
//! # Encrypt a key
//! stomata web3 key encrypt -n my-secret
//! ```
//!
//! # Modules
//!
//! - [`cli`] - Command-line interface definitions and argument parsing
//! - [`web3_feature`] - Core Web3 functionality implementations
pub mod cli;
pub mod web3_feature;
