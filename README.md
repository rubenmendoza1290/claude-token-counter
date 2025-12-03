# Claude Token Counter

A powerful command-line tool to monitor and visualize your Claude API token usage and track costs in real-time.

## Overview

Claude Token Counter helps developers understand their Claude usage through two complementary approaches:

1. **Local File Monitoring** (Works for everyone, including Claude Pro users): Analyzes Claude Code's local JSONL logs to provide real-time token usage statistics and cost estimates.

2. **API-Based Tracking** (Requires Team/Enterprise Admin key): Fetches official usage data directly from Anthropic's Usage API for organization-wide insights.

This dual approach ensures the tool is valuable whether you're an individual developer with a Pro account or part of an enterprise team.

## Current Status

**Beta** - Core features implemented and tested. The live monitoring feature is fully functional and has been validated with real-world usage data (38M+ tokens tracked).

## Key Features

- **Real-Time Live Monitoring**: Watch your Claude Code token usage update automatically as you work
- **Comprehensive Token Tracking**: Monitors input, output, cache creation, and cache read tokens
- **Accurate Cost Estimation**: Calculates costs based on official Claude Sonnet 4.5 pricing
- **Beautiful Terminal Display**: Color-coded output with formatted numbers and clear statistics
- **API Usage Tracking**: View current status and historical usage for Team/Enterprise accounts
- **Secure Configuration**: API keys stored safely in your user config directory

## Installation

### Build from Source

```bash
# Clone the repository
git clone https://github.com/rubenmendoza1290/claude-token-counter.git
cd claude-token-counter

# Build release binary
cargo build --release

# The optimized binary will be at target/release/claude-token-counter
```

### Add to PATH (Optional)

```bash
# Copy to a directory in your PATH
cp target/release/claude-token-counter /usr/local/bin/

# Or create a symlink
ln -s $(pwd)/target/release/claude-token-counter /usr/local/bin/claude-token-counter
```

## Usage

### Live Monitor (Recommended - Works for Everyone)

Monitor your Claude Code token usage in real-time by analyzing local JSONL logs:

```bash
# Start live monitoring with default 2-second refresh
claude-token-counter live

# Custom refresh interval (in seconds)
claude-token-counter live --refresh 5
```

The live monitor displays:
- Total tokens used (input, output, cache creation, cache read)
- Number of messages processed
- Estimated cost based on current Anthropic pricing
- Auto-refreshing statistics

**Note**: This feature reads from `~/.claude/projects/` where Claude Code stores conversation logs.

### API-Based Commands (Requires Admin Key)

For Team/Enterprise users with Admin API keys:

#### Configure API Key

```bash
claude-token-counter config --api-key YOUR_ADMIN_API_KEY
```

#### Check Current Status

```bash
claude-token-counter status
```

Shows current token usage and remaining quota for the billing period.

#### View Usage History

```bash
# Last 30 days (default)
claude-token-counter history

# Custom number of days
claude-token-counter history --days 7
```

## Architecture

Built with Rust for performance and reliability:

- **CLI Framework**: Clap with derive macros for type-safe argument parsing
- **Async Runtime**: Tokio for efficient async operations
- **HTTP Client**: Reqwest for Anthropic API communication
- **Terminal UI**: Colored and crossterm for beautiful terminal output
- **File Parsing**: Serde JSON for parsing Claude Code JSONL logs
- **Configuration**: Secure storage in `~/.config/claude-token-counter/`

## Next Milestones

- Implement file-watching for instant updates (eliminate polling)
- Add filtering by date range, project, and model
- Export usage data to CSV/JSON
- Model-specific cost breakdowns
- Configurable usage alerts and thresholds
- Cross-platform testing (Linux, Windows)
- Automated GitHub releases with pre-built binaries

## Development

See [CLAUDE.md](CLAUDE.md) for detailed development guidelines and architecture documentation.

### Quick Start

```bash
# Run in development mode
cargo run -- live

# Run tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Pricing Reference

The tool uses current Claude Sonnet 4.5 pricing for cost estimation:

- Input tokens: $3.00 per million
- Output tokens: $15.00 per million
- Cache write tokens: $3.75 per million
- Cache read tokens: $0.30 per million

## License

MIT

## Contributing

Contributions welcome! Please ensure all tests pass and code is formatted with `cargo fmt` before submitting PRs.
