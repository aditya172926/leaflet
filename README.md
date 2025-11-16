<div align="center">
  <h1>Stomata</h1>

  <p>
    A CLI tool for **collecting and visualizing system metrics** in real-time. It provides a lightweight, terminal-based dashboard to monitor memory, swap, CPU, and other resources with a clean interface.
  </p>

</div>

<div align="center">
<img src="https://github.com/user-attachments/assets/411bd137-186e-4276-bac1-e586c87c7761" alt="Quick Demo showing stomata cli" />
</div>

> **Note**: Stomata is currently under development and more features and fixes are upcoming.
Right now Stomata is very minimalistic and will introduce new features as the development goes on.

---

## Features

- Real-time **memory and swap usage gauges**  
- Minimal dependencies; built with **Rust**
- Designed as a **workspace**: includes a reusable library (`Stomata-core`) and a CLI (`Stomata-cli`)  

---

## Installation

Install via **crates.io**:

```bash
cargo install stomata-cli
```

You can also add the stomata-core crate in your rust project as it is deployed as a separate crate

```bash
cargo add stomata-core
```

## Building Locally with Cargo
Make sure you have Rust installed on your machine. Stomata is being built using rustc version 1.90.0

Clone the master branch of the repository
```bash
git clone https://github.com/aditya172926/stomata-cli.git
```

Once cloned you can either run `cargo build` or `make build` to execute debug build command from the make file. If you want to build the release execute `make release`

To run the cli, use the command
```bash
stomata --interval 1000
```
interval is an optional arguement to give. It is the refresh rate of the metrics to be fetched and rendered on the terminal UI.

## Contributing

Contributions are welcome! Please read our [Contributing Guidelines](CONTRIBUTING.md) before submitting a pull request.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE))
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT))