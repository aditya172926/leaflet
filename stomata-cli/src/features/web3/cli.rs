use clap::{Parser, Subcommand};

/// Web3 developer tools CLI
#[derive(Parser, Clone)]
#[command(name = "web3")]
#[command(about = "Web3 developer tools")]
pub struct Web3Cli {
    #[command(subcommand)]
    pub tool: Web3Tool,
}

#[derive(Subcommand, Clone)]
pub enum Web3Tool {
    #[command(name = "address-validator", alias = "av")]
    AddressValidator {
        /// Ethereum address to validate
        #[arg(short, long, required = true)]
        address: String,
    },
    #[command(name = "encrypt-key", alias = "ek")]
    EncryptKey {
        // Key to encrypt
        #[arg(short, long, required = true)]
        key: String,
    },
}
