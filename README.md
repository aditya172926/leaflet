# Leaflet

> **Note**: Leaflet is currently under development and more features and fixes are upcoming 

**Leaflet** is a CLI tool for **collecting and visualizing system metrics** in real-time. It provides a lightweight, terminal-based dashboard to monitor memory, swap, CPU, and other resources with a clean interface.
Right now Leaflet is very minimalistic and will introduct new features as the development goes on.

---

## Features

- Real-time **memory and swap usage gauges**  
- Minimal dependencies; built with **Rust**
- Designed as a **workspace**: includes a reusable library (`leaflet-core`) and a CLI (`leaflet-cli`)  

---

## Installation

Install via **crates.io**:

```bash
cargo install leaflet-cli
```

You can also add the leaflet-core crate in your rust project as it is deployed as a separate crate

```bash
cargo add leaflet-core
```

## Building Locally with Cargo
Make sure you have Rust installed on your machine. Leaflet is being built using rustc version 1.90.0

Clone the master branch of the repository
```bash
git clone https://github.com/aditya172926/leaflet.git
```

Once cloned you can either run `cargo build` or `make build` to execute debug build command from the make file. If you want to build the release execute `make release`

To run the cli, use the command
```bash
leaflet --interval 1000
```
interval is an optional arguement to give. It is the refresh rate of the metrics to be fetched and rendered on the terminal UI.

## How it looks?
Right now Leaflet just tracks Memory consumption and swap consumption and displays it in a TUI. This is what it looks like


## Contributing
This is a very early stage repository. But feel free to suggest features and make issues before starting to work on a pull request.

### Contributing guidelines
- Fork the repository before starting to work and making a PR.
- PRs made should be related and linked to an existing issue.
- Before making a new issue check if it is not being duplicated.
- Ask for issues to be assigned to you before starting the work.
- Work only on the issues assigned to you, to avoid other people working on the same issue.
- A contributor may be assigned to only one issue at a time, unless it is blocked by other dependent issues related to the main issue that contributor wants to work on.