## Web3 lib

This lib can be used with Stomata-CLI for interactive display and interaction.

This crate has interactive and non-interactive features that users can use.
These will be listed down here.

The difference between interactive features and non-interactive features is that cli renders a TUI for interactive feature, and non-interactive features would just print the results in the user's terminal and exit.

## Non-interactive features
- EVM Address validation
To validate an EVM address and get its checksummed format.
```
stomata web3 av --address 0x...
```
This cmd returns either a valid checksummed address or an error for Invalid address with incorrect length or hex characters.
Implemented EIP-55