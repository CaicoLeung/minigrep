# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`minigrep` is a simple command-line grep-like tool written in Rust. It searches for a query string in a file, with optional case-insensitive matching via environment variable.

## Commands

```bash
# Build and run
cargo run -- <query> <file_path>

# Run tests
cargo test

# Format code (run before committing)
cargo fmt

# Lint
cargo clippy

# Release build
cargo build --release
```

## Environment Variables

- `IGNORE_CASE` - Set to `yes`, `true`, or `y` for case-insensitive search

## Architecture

**Entry point:** `src/main.rs` — parses CLI arguments, handles errors, exits with status codes.

**Core logic:** `src/lib.rs` — contains:
- `Config<'a>` struct with `query`, `file_path`, and `ignore_case` fields
- `Config::build(args)` — parses CLI arguments and reads `IGNORE_CASE` env var
- `search(query, contents)` — case-sensitive line search
- `search_case_insensitive(query, contents)` — case-insensitive line search
- `run(Config)` — reads file, executes search, prints results

**Tests:** Inline in `lib.rs` under `#[cfg(test)]` modules.

## Notes

- The `CONTEXTS` test constant was moved into the tests module to keep it test-scoped.
- The binary exits with status code 1 on argument parsing errors or runtime errors.
