use clap::{Parser, Subcommand, ValueEnum};

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
    #[command(subcommand)]
    Key(KeySubCommands),
}

#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    Hex,
    Utf8,
}

#[derive(Subcommand, Clone)]
pub enum KeySubCommands {
    #[command(name = "encrypt", alias = "e")]
    Encrypt {
        // Name of the key to encrypt
        #[arg(short, long, required = true)]
        name: String,
    },
    #[command(name = "decrypt", alias = "d")]
    Decrypt {
        // Name of the key to decrypt
        #[arg(short, long, required = true)]
        name: String,

        #[arg(short, long, value_enum, default_value_t = OutputFormat::Utf8)]
        format: OutputFormat,
    },
    #[command(name = "list", alias = "l")]
    List {},
    #[command(name = "delete", alias = "del")]
    Delete {
        // Name of the key to decrypt
        #[arg(short, long, required = true)]
        name: String,
    },
}
