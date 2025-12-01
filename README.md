# Claude Token Counter

A command-line tool to visualize Claude API token usage and track your monthly subscription limits.

## Features

- View current token usage and remaining quota
- Track usage history over time
- Beautiful terminal visualizations
- Secure API key storage

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/claude-token-counter`.

## Usage

### Configure API Key

```bash
claude-token-counter config --api-key YOUR_API_KEY
```

### Check Current Status

```bash
claude-token-counter status
```

### View Usage History

```bash
# Last 30 days (default)
claude-token-counter history

# Custom number of days
claude-token-counter history --days 7
```

## Development

See [CLAUDE.md](CLAUDE.md) for development guidelines.
