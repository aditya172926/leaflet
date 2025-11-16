# Contributing to Stomata

Thank you for your interest in contributing to Stomata! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Contribution Guidelines](#contribution-guidelines)
- [Code Style](#code-style)
- [Commit Messages](#commit-messages)
- [Pull Request Process](#pull-request-process)
- [License](#license)

## Code of Conduct

This project is in its early stages, but committed to providing a welcoming and inclusive environment. Please be respectful and constructive in all interactions.

## Getting Started

Stomata is a system monitoring CLI tool written in Rust. Before contributing:

1. Familiarize yourself with the project by reading the [README](README.md)
2. Browse existing [issues](https://github.com/aditya172926/stomata-cli/issues) to see what needs work

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue with:

- **Clear title** describing the problem
- **Steps to reproduce** the issue
- **Expected behavior** vs actual behavior
- **Environment details** (OS, Rust version, Stomata version)
- **Screenshots or logs** if applicable

### Suggesting Features

Feature suggestions are welcome! First discuss the feature and please create an issue with:

- **Clear description** of the feature
- **Use case** - why is this feature needed?
- **Proposed implementation** (if you have ideas)
- **Alternatives considered**

### Asking Questions

If you have questions about using Stomata, please:

- Check existing issues and discussions first
- Create a new issue with the `question` label
- Be specific about what you're trying to accomplish

## Development Setup

### Prerequisites

- Rust 1.90 or later (edition 2024)
- Git

### Setup Steps

1. **Fork the repository** on GitHub

2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/stomata-cli.git
   cd stomata-cli
   ```

3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/aditya172926/stomata-cli.git
   ```

4. **Build the project**:
   ```bash
   cargo build
   ```

5. **Run the CLI**:
   ```bash
   ./target/debug/stomata
   ```

## Contribution Guidelines

### Before You Start

1. **Check for existing issues** - avoid duplicate work
2. **Create or find an issue** - all PRs should be linked to an issue
3. **Ask to be assigned** - comment on the issue asking to be assigned
4. **Wait for assignment** - don't start work until assigned to avoid conflicts
5. **One issue at a time** - work on one issue unless multiple are related/blocking

### Working on Your Contribution

1. **Create a new branch** from `master`:
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/bug-description
   ```

2. **Make your changes**:
   - Write clear, focused commits
   - Test your changes locally
   - Follow the code style guidelines

3. **Keep your branch updated**:
   ```bash
   git fetch upstream
   git rebase upstream/master
   ```

4. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

## Code Style

### Formatting

- **Run `cargo fmt`** before committing:
  ```bash
  cargo fmt --all
  ```

### Before making a PR

- **Run `make build`** which will do the fmt, check and build:
  ```bash
  make build
  ```

### Best Practices

- Use descriptive variable names
- Add comments for complex logic
- Keep functions small and focused
- Avoid unwrap() - use proper error handling with `anyhow` or `Result`

### Code Organization

- `stomata-core/` - Core library with system monitoring logic
- `stomata-cli/` - CLI application and UI components

## Commit Messages

Write clear, descriptive commit messages following this format:

```
<type>: <short summary>

<optional detailed description>

<optional footer with issue references>
```

### Types

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, no logic change)
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

### Examples

```
feat: add memory usage chart to dashboard

Implements a real-time memory usage visualization using
ratatui's chart widgets.

Closes #42
```

```
fix: prevent crash when no network interfaces found

Handle the case where sysinfo returns an empty interface list
on some systems.

Fixes #38
```

## Pull Request Process

1. **Ensure your code follows all guidelines** above

2. **Run make build** before submitting a PR:
   - The cmd makes a debug build, formats the code and checks for compile errors

3. **Create the Pull Request**:
   - Use a clear, descriptive title
   - Reference the issue number: "Closes #123" or "Fixes #123"
   - Describe what changes you made and why
   - Add screenshots for UI changes
   - Mark as draft if work in progress

4. **Respond to feedback**:
   - Address review comments promptly
   - Push new commits or amend existing ones
   - Request re-review when ready

5. **Merge**:
   - Maintainers will merge when approved
   - Your branch will be deleted automatically

### PR Checklist

Before submitting, ensure:

- [ ] Code compiles without errors
- [ ] `make build` has been run
- [ ] Documentation is updated
- [ ] Commit messages are clear
- [ ] PR description references the issue
- [ ] Screenshots included for UI changes (if any)

## License

By contributing to Stomata, you agree that your contributions will be licensed under the project's dual license:

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed under MIT OR Apache-2.0, without any additional terms or conditions.

See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.

---

## Questions?

If you have questions about contributing, feel free to:

- Open an issue with the `question` label
- Reach out to the maintainers
- Check existing discussions

Thank you for contributing to Stomata! 🚀