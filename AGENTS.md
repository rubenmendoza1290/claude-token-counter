# AGENTS.md

This file provides guidance to AI coding assistants when working with code in this repository.

## Project Overview

Claude Token Counter is a Rust-based CLI tool that helps users visualize their Claude API token usage and track their monthly subscription limits. The tool fetches usage data from the Anthropic API and presents it in a user-friendly terminal interface.

## Current Workflow Phase

**Phase**: Foundation & Setup
**Status**: Complete - CLI skeleton established, ready for core implementation

### Workflow Checklist

- [x] Project initialization with Cargo
- [x] Dependency configuration
- [x] Basic CLI structure with subcommands
- [x] Documentation suite (README, development guide)
- [x] Git repository setup and GitHub integration
- [x] SSH authentication configuration
- [ ] Configuration module implementation
- [ ] API client module implementation
- [ ] Data models definition
- [ ] Status command implementation
- [ ] History command implementation
- [ ] Config command implementation
- [ ] Error handling and validation
- [ ] Unit and integration tests
- [ ] Cross-platform testing

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

## Key Decisions & Context

### Idea & Validation

**Core Idea**: Provide developers with a simple, terminal-based tool to monitor Claude API token consumption and stay within subscription limits.

**Target Audience**: Developers and power users who interact with the Claude API and want quick visibility into their usage without navigating web dashboards.

**Validation Status**: Project is in initial implementation phase. The need is clear for API users who want command-line workflow integration.

### Technical Decisions

**Language Choice**: Rust selected for performance, safety, excellent CLI ecosystem, and ability to distribute as single binary.

**Architecture Pattern**: Modular design with clear separation between CLI parsing, API interaction, configuration management, and display logic.

**Async Approach**: Tokio-based async/await for non-blocking API calls and potential future concurrent operations.

**Configuration Strategy**: File-based config storage in standard user config directory with security-first design (restricted permissions, no credential logging).

### Creative Strategy

**User Experience**: Focus on clarity and simplicity - three focused subcommands (status, history, config) that each do one thing well.

**Visual Design**: Terminal-native styling using colored output, ASCII-based visualizations where appropriate, and clean tabular data presentation.

**Security Model**: Never store credentials in repository, use file system permissions, avoid credential exposure in logs or error messages.

## Session History

### Session 2025-12-01
- **Phase**: Foundation & Setup
- **Accomplishments**:
  - Created new Cargo project with complete dependency configuration
  - Implemented CLI skeleton with three subcommands (status, history, config)
  - Created comprehensive documentation suite (README.md, CLAUDE.md, AGENTS.md, GEMINI.md)
  - Configured .gitignore for Rust projects
  - Initialized Git repository and pushed to GitHub
  - Set up SSH authentication with GitHub using Ed25519 key
- **Key Decisions**:
  - Chose Rust with Tokio async runtime for implementation
  - Selected Clap with derive macros for CLI parsing
  - Decided on ~/.config/ for cross-platform config storage
  - Designed modular architecture with planned separation of concerns
- **Next Steps**:
  - Implement configuration module with secure file persistence
  - Build API client for Anthropic API integration
  - Create data models for API responses
  - Implement actual functionality for each subcommand

## Working Instructions

### Current Focus

The immediate priority is implementing core functionality now that the foundation is established. Start with the configuration module since API key storage is required for all other operations.

### Development Workflow

1. **Before Coding**: Review AGENTS.md and session summary to understand current state
2. **Implementation**: Follow the module structure outlined in Architecture section
3. **Testing**: Write tests alongside implementation
4. **Documentation**: Update README and AGENTS.md as features are completed
5. **Commits**: Make atomic commits with clear messages describing what was implemented

### Code Style Guidelines

- Follow standard Rust formatting (cargo fmt)
- Address all clippy warnings
- Use descriptive variable names
- Add doc comments for public functions and modules
- Prefer explicit error handling over unwrap/expect

### Security Considerations

- API keys are stored in `~/.config/claude-token-counter/config.json`
- This file should have restricted permissions (0600)
- Never log or display full API keys
- The config file is gitignored to prevent accidental commits

## Repository Information

- **GitHub**: https://github.com/rubenmendoza1290/claude-token-counter
- **Remote**: git@github.com:rubenmendoza1290/claude-token-counter.git
- **Branch**: main
- **License**: MIT
