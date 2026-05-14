# Tropical Project Manager

A Rust-based Terminal User Interface (TUI) application designed for managing multiple software projects. It scans a master directory for Git repositories and provides a consolidated view of their status (branch, dirty state, uncommitted files).

## Project Overview

- **Main Technologies:** Rust, `ratatui` (TUI framework), `crossterm` (terminal handling), `git2` (Git integration).
- **Core Functionality:**
    - Scans `K:/source` (hardcoded in `src/main.rs`) for Git repositories.
    - Displays project name, current branch, and status (dirty/clean, number of uncommitted files).
    - Provides a TUI for navigating and interacting with the project list.
- **Architecture:** A single-binary Rust application with a simple App/State/View pattern typical for `ratatui` applications.

## Building and Running

### Prerequisites
- [Rust toolchain](https://rustup.rs/) (2024 edition or later).

### Commands
- **Run:** `cargo run`
- **Build (Release):** `cargo build --release`
- **Test:** `cargo test` (Note: No tests found in initial scan; TODO: add unit tests).
- **Lint:** `cargo clippy`

## Development Conventions

- **Hardcoded Paths:** The `MASTER_DIR` is currently hardcoded as `K:/source` in `src/main.rs`. Future development should consider making this configurable.
- **TUI Style:** Uses `ratatui` with standard layout constraints. Adhere to existing styling in `src/main.rs` when adding new UI elements.
- **Git Integration:** Uses `git2-rs`. Ensure proper error handling when opening or querying repositories.
- **Template Heritage:** This project appears to be initialized from `jules_dev_standard`. Refer to `jules_dev_standard/FMG-REPO-BIBLE.md` for broader organizational standards if applicable.

## Key Files
- `src/main.rs`: Entry point and main logic (TUI, scanning, state management).
- `Cargo.toml`: Project dependencies and metadata.
- `PROJECT-TYPES-GUIDE.md`: Comprehensive guide for different project types (likely from template).
