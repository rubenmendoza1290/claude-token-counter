# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Claude Token Counter is a Rust-based CLI tool that helps users monitor their Claude token usage through two approaches: (1) Real-time parsing of local Claude Code JSONL logs for all users, and (2) API-based tracking for Team/Enterprise users with Admin keys. The tool provides accurate cost estimation, beautiful terminal visualization, and comprehensive token tracking across all token types (input, output, cache creation, cache read).

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
cargo run -- live                    # Live monitoring (recommended)
cargo run -- live --refresh 5        # Custom refresh interval
cargo run -- status                  # API-based current status
cargo run -- history --days 7        # API-based history
cargo run -- config --api-key KEY    # Configure API key

# Run the built binary
./target/release/claude-token-counter live
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
cargo run -- live

# Watch mode (requires cargo-watch)
cargo watch -x "run -- live"
```

## Architecture

### High-Level Design

The application follows a modular architecture with dual data sources:

1. **CLI Layer** (`main.rs`): Command-line interface using `clap` with derive macros. Defines four subcommands (status, history, config, live) and handles argument parsing. Contains live monitor implementation with terminal control.

2. **Local Module** (`src/local/`): Parses Claude Code JSONL logs from `~/.claude/projects/`. Recursively discovers all JSONL files, deserializes log entries, extracts token usage data, and aggregates statistics. Works for all users without requiring API keys.

3. **API Client Module** (`src/api/`): Handles communication with Anthropic Usage API. Supports two endpoints: `/v1/organizations/usage_report/messages` (regular API) and `/v1/organizations/usage_report/claude_code` (Claude Code specific). Requires Admin API keys (Team/Enterprise only).

4. **Configuration Module** (`src/config/`): Manages API key storage and user preferences. Stores config in `~/.config/claude-token-counter/config.json` using the `dirs` crate for cross-platform home directory resolution.

5. **Display Module** (`src/display/`): Formats and presents data to the terminal using `colored` for styled output. Includes status display, history visualization, progress bars, and formatted tables.

6. **Models Module** (`src/models/`): Serde-based structs for API responses including UsageRecord, UsageResponse, and UsageSummary for aggregated statistics.

### Key Design Decisions

- **Dual-Mode Architecture**: Supports both local file parsing (universal) and API integration (enterprise), making the tool valuable for all user types.
- **Local-First Approach**: Prioritizes local JSONL parsing as the primary feature since it works for all users and provides more granular data.
- **Async Runtime**: Uses Tokio for async operations to handle API requests and non-blocking sleep in live monitor.
- **Real-Time Monitoring**: Implements live refresh using crossterm for terminal control and tokio::sleep for intervals.
- **Comprehensive Token Tracking**: Tracks all token types (input, output, cache creation, cache read) for accurate cost calculation.
- **Error Handling**: Uses `anyhow` for flexible error handling with context. Gracefully handles malformed JSONL lines.
- **Config Storage**: Stores API keys securely in user's config directory (never in the repository).
- **CLI Framework**: Uses `clap` with derive macros for type-safe, self-documenting CLI.

### Anthropic API Integration

When working with the API:
- Base URL: `https://api.anthropic.com/v1`
- Usage endpoints:
  - `/v1/organizations/usage_report/messages` - Regular API usage
  - `/v1/organizations/usage_report/claude_code` - Claude Code usage
- Requires Admin API key in `x-api-key` header
- API version header: `anthropic-version: 2023-06-01`
- Date range support with ISO 8601 format (YYYY-MM-DD)
- Only available for Team/Enterprise accounts

### Local JSONL Parsing

Claude Code stores conversation logs at `~/.claude/projects/` with the following structure:

```json
{
  "message": {
    "model": "claude-sonnet-4.5",
    "usage": {
      "input_tokens": 1234,
      "output_tokens": 567,
      "cache_creation_input_tokens": 890,
      "cache_read_input_tokens": 123
    }
  },
  "timestamp": "2025-12-02T10:30:00Z",
  "agentId": "..."
}
```

The local module:
- Recursively walks `~/.claude/projects/` to find all `.jsonl` files
- Parses each line as JSON, skipping malformed entries with warnings
- Aggregates token usage across all files and projects
- Provides real-time monitoring with configurable refresh intervals

### Module Structure

```
src/
├── main.rs           # Entry point, CLI definition, live monitor
├── api/
│   └── mod.rs       # API client implementation
├── config/
│   └── mod.rs       # Config persistence and loading
├── display/
│   └── mod.rs       # Terminal output formatting
├── local/
│   └── mod.rs       # JSONL parsing and aggregation
└── models/
    └── mod.rs       # Data models for API responses
```

### Dependencies Rationale

- `clap 4.5`: Industry-standard CLI framework with excellent derive support
- `tokio 1.40`: De facto async runtime for Rust
- `reqwest 0.12`: High-level HTTP client built on tokio
- `serde 1.0` + `serde_json`: Standard serialization/deserialization
- `chrono 0.4`: Date/time handling for usage history and timestamps
- `colored 2.1`: Terminal color and styling
- `dirs 5.0`: Cross-platform config directory resolution
- `anyhow 1.0`: Ergonomic error handling for applications
- `notify 7.0`: File system watching (prepared for future instant updates)
- `walkdir 2.5`: Recursive directory traversal for finding JSONL files
- `crossterm 0.28`: Terminal control for live monitor screen clearing

## Implementation Details

### Live Monitor

The live monitor (`cargo run -- live`) is the flagship feature:

1. Discovers all JSONL files in `~/.claude/projects/`
2. Parses each file to extract token usage
3. Aggregates totals across all files
4. Calculates accurate costs using Claude Sonnet 4.5 pricing:
   - Input: $3/MTok
   - Output: $15/MTok
   - Cache write: $3.75/MTok
   - Cache read: $0.30/MTok
5. Displays formatted output with colors and number formatting
6. Refreshes automatically at configurable intervals (default 2 seconds)
7. Uses crossterm to clear screen and reposition cursor for smooth updates

### Cost Calculation

```rust
fn calculate_cost(usage: &AggregatedUsage) -> f64 {
    let input_cost = (usage.total_input as f64 / 1_000_000.0) * 3.0;
    let output_cost = (usage.total_output as f64 / 1_000_000.0) * 15.0;
    let cache_write_cost = (usage.total_cache_creation as f64 / 1_000_000.0) * 3.75;
    let cache_read_cost = (usage.total_cache_read as f64 / 1_000_000.0) * 0.30;

    input_cost + output_cost + cache_write_cost + cache_read_cost
}
```

### Number Formatting

The tool uses comma separators for readability:

```rust
fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let mut count = 0;

    for c in s.chars().rev() {
        if count == 3 {
            result.insert(0, ',');
            count = 0;
        }
        result.insert(0, c);
        count += 1;
    }

    result
}
```

## Current Implementation Status

### Completed Features

- [x] CLI structure with all subcommands
- [x] Local JSONL parsing module
- [x] Live monitoring with real-time updates
- [x] Accurate cost calculation
- [x] Beautiful terminal formatting
- [x] API client for Anthropic Usage API
- [x] Configuration module with file persistence
- [x] Display module with colored output
- [x] Data models for API responses
- [x] Recursive directory walking
- [x] Graceful error handling
- [x] Number formatting with commas

### Known Limitations

- Live monitor uses polling (refresh interval) rather than file watching
- No filtering by date, project, or model yet
- No export functionality (CSV/JSON)
- Not yet tested on Windows or Linux
- No automated releases or pre-built binaries

## Security Considerations

- API keys are stored in `~/.config/claude-token-counter/config.json`
- This file should have restricted permissions (0600)
- Never log or display full API keys
- The config file is gitignored to prevent accidental commits
- Local JSONL files may contain conversation data - handle with care

## Testing Notes

The live monitor has been validated with real data:
- Successfully parsed 38M+ tokens across multiple projects
- Tested with various JSONL formats and edge cases
- Gracefully handles malformed JSON lines
- Works with Claude Code projects directory structure

## Future Development

Priority enhancements:

1. **File Watching**: Replace polling with `notify` crate for instant updates
2. **Filtering**: Add date range, project, and model filtering
3. **Export**: CSV/JSON export for external analysis
4. **Model Breakdown**: Per-model usage statistics
5. **Alerts**: Configurable usage thresholds
6. **Historical Analysis**: Daily/weekly/monthly breakdowns from local files
7. **Cross-Platform Testing**: Validate on Linux and Windows
8. **CI/CD**: GitHub Actions for automated builds and releases
