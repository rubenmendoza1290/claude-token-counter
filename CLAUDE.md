# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Claude Token Counter is a Rust-based CLI tool that helps users visualize their Claude API token usage and track their monthly subscription limits. The tool fetches usage data from the Anthropic API and presents it in a user-friendly terminal interface.

## Build and Development Commands

### Building
```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# The binary will be at target/release/claude-token-counter
```

### Running
```bash
# Run directly with cargo
cargo run -- status
cargo run -- history --days 7
cargo run -- config --api-key YOUR_KEY

# Run the built binary
./target/release/claude-token-counter status
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Quality
```bash
# Format code
cargo fmt

# Check formatting without modifying
cargo fmt -- --check

# Run clippy (linter)
cargo clippy

# Run clippy with all warnings
cargo clippy -- -W clippy::all
```

### Development Workflow
```bash
# Check code compiles without building
cargo check

# Build and run in one step
cargo run

# Watch mode (requires cargo-watch)
cargo watch -x run
```

## Architecture

### High-Level Design

The application follows a modular architecture with clear separation of concerns:

1. **CLI Layer** (`main.rs`): Command-line interface using `clap` with derive macros. Defines subcommands (status, history, config) and handles argument parsing.

2. **API Client Module** (planned): Handles communication with the Anthropic Claude API to fetch usage data. Uses `reqwest` for HTTP requests and `tokio` for async operations.

3. **Configuration Module** (planned): Manages API key storage and user preferences. Stores config in `~/.config/claude-token-counter/config.json` using the `dirs` crate for cross-platform home directory resolution.

4. **Display Module** (planned): Formats and presents data to the terminal using `colored` for styled output. Will include progress bars, usage graphs, and formatted tables.

5. **Data Models** (planned): Serde-based structs for deserializing API responses and managing usage data.

### Key Design Decisions

- **Async Runtime**: Uses Tokio for async operations to handle API requests efficiently
- **Error Handling**: Uses `anyhow` for flexible error handling with context
- **Config Storage**: Stores API keys securely in user's config directory (never in the repository)
- **CLI Framework**: Uses `clap` with derive macros for type-safe, self-documenting CLI

### Anthropic API Integration

When implementing API integration:
- The Anthropic API base URL is: `https://api.anthropic.com/v1`
- Usage data endpoint: `/v1/usage` (check current API documentation)
- Requires `x-api-key` header for authentication
- API responses are JSON format, use `serde_json` for parsing
- Rate limits apply - implement exponential backoff for retries
- Store API key in config file, never hardcode or commit it

### Module Structure (Planned)

```
src/
├── main.rs           # Entry point, CLI definition
├── api/
│   ├── mod.rs       # API client module
│   └── client.rs    # HTTP client implementation
├── config/
│   ├── mod.rs       # Config module
│   └── store.rs     # Config persistence
├── display/
│   ├── mod.rs       # Display module
│   ├── status.rs    # Status display
│   └── history.rs   # History visualization
└── models/
    ├── mod.rs       # Data models
    ├── usage.rs     # Usage data structures
    └── config.rs    # Config data structures
```

### Dependencies Rationale

- `clap`: Industry-standard CLI framework with excellent derive support
- `tokio`: De facto async runtime for Rust
- `reqwest`: High-level HTTP client built on tokio
- `serde/serde_json`: Standard serialization/deserialization
- `chrono`: Date/time handling for usage history
- `colored`: Terminal color and styling
- `dirs`: Cross-platform config directory resolution
- `anyhow`: Ergonomic error handling for applications

## Security Considerations

- API keys are stored in `~/.config/claude-token-counter/config.json`
- This file should have restricted permissions (0600)
- Never log or display full API keys
- The config file is gitignored to prevent accidental commits
