# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`minigrep` is a simple command-line grep-like tool written in Rust. It searches for a query string in a file, with colored output and line numbers.

## Commands

```bash
# Build and run
cargo run -- <query> <file_path>

# Case-insensitive search
cargo run -- <query> <file_path> -i
cargo run -- <query> <file_path> --ignore-case

# Run tests
cargo test

# Format code (run before committing)
cargo fmt

# Lint
cargo clippy

# Quick compile check (faster than build)
cargo check

# Release build
cargo build --release
```

## Architecture

**Entry point:** `src/main.rs` — minimal (~11 lines). Uses clap's `Parser` derive to parse arguments into `Config`, calls `run()`, exits with status code 1 on error.

**Core logic:** `src/lib.rs` — contains:
- `Config` struct — derives `Parser` from clap, with `query`, `file_path` (PathBuf), and `ignore_case` (bool via `-i`/`--ignore-case` flag)
- `run(Config)` — reads file, delegates to search functions, prints colored results
- `search(query, contents)` — case-sensitive line search, returns formatted strings with line numbers and highlighted matches
- `search_case_insensitive(query, contents)` — case-insensitive variant
- `format_line((line, content), query, ignore_case)` — helper that adds line numbers and colors matches red, line numbers green

**Tests:** Inline in `lib.rs` under `#[cfg(test)]` module with `CONTEXTS` test constant.

**Dependencies:**
- `clap` 4.6 — CLI argument parsing via derive macro
- `colored` 3.1 — Terminal colors (green for line numbers, red for matches)

**Output format:** Each matching line is printed as `<line_number>: <content>` where the query substring is highlighted in red.
