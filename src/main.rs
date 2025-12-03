use clap::{Parser, Subcommand};
use anyhow::{Context, Result};

mod api;
mod config;
mod display;
mod local;
mod models;

#[derive(Parser)]
#[command(name = "claude-token-counter")]
#[command(about = "A CLI tool to visualize Claude API token usage and track monthly subscription limits", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Display current token usage and remaining quota
    Status,
    /// Show usage history over time
    History {
        /// Number of days to show (default: 30)
        #[arg(short, long, default_value_t = 30)]
        days: u32,
    },
    /// Configure API key and subscription details
    Config {
        /// Claude API key
        #[arg(long)]
        api_key: Option<String>,
    },
    /// Monitor Claude Code token usage in real-time from local JSONL files
    Live {
        /// Refresh interval in seconds (default: 2)
        #[arg(short, long, default_value_t = 2)]
        refresh: u64,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Status => {
            // Load config to get API key
            let config = config::Config::load()
                .context("No API key configured. Run 'config --api-key YOUR_KEY' first")?;

            // Create API client
            let client = api::AnthropicClient::new(config.api_key)?;

            println!("Fetching usage data from Anthropic API...");

            // Fetch usage data (last 30 days)
            let usage_response = client.fetch_usage(30).await?;

            // Calculate summary
            let summary = models::UsageSummary::from_records(&usage_response.data);

            // Display results with beautiful formatting
            // Note: Set your monthly limit here (in tokens)
            // For Claude Pro: typically 5M tokens/month
            let monthly_limit = Some(5_000_000); // Adjust this to your actual limit
            display::display_status(&summary, monthly_limit);
        }
        Commands::History { days } => {
            // Load config to get API key
            let config = config::Config::load()
                .context("No API key configured. Run 'config --api-key YOUR_KEY' first")?;

            // Create API client
            let client = api::AnthropicClient::new(config.api_key)?;

            println!("Fetching usage history from Anthropic API...");

            // Fetch usage data
            let usage_response = client.fetch_usage(days).await?;

            // Display history
            display::display_history(&usage_response.data, days);
        }
        Commands::Config { api_key } => {
            if let Some(key) = api_key {
                let config = config::Config::new(key);
                config.save()?;
                println!("✓ API key configured successfully");
            } else {
                // Show current config status
                match config::Config::load() {
                    Ok(cfg) => {
                        let masked_key = format!("{}...{}",
                            &cfg.api_key[..8],
                            &cfg.api_key[cfg.api_key.len()-4..]
                        );
                        println!("API key is configured: {}", masked_key);
                    }
                    Err(_) => {
                        println!("No API key configured. Use --api-key to set one.");
                    }
                }
            }
        }
        Commands::Live { refresh } => {
            // Run live monitoring
            run_live_monitor(refresh).await?;
        }
    }

    Ok(())
}

/// Run live monitoring of Claude Code token usage
async fn run_live_monitor(refresh_seconds: u64) -> Result<()> {
    use colored::*;
    use crossterm::{
        cursor,
        terminal::{self, ClearType},
        ExecutableCommand,
    };
    use std::io::stdout;
    use std::time::Duration;

    println!("{}", "Starting Claude Code Live Monitor...".bright_cyan().bold());
    println!("Reading from: ~/.claude/projects/\n");
    println!("Press Ctrl+C to exit\n");

    tokio::time::sleep(Duration::from_secs(1)).await;

    loop {
        // Parse all JSONL files
        let usage = local::parse_all_files()?;

        // Clear screen and move cursor to top
        stdout()
            .execute(cursor::MoveTo(0, 0))?
            .execute(terminal::Clear(ClearType::FromCursorDown))?;

        // Display header
        println!("{}", "═".repeat(70).bright_blue());
        println!("{}", "  CLAUDE CODE - LIVE TOKEN USAGE".bright_cyan().bold());
        println!("{}", "═".repeat(70).bright_blue());
        println!();

        // Token counts
        println!("{}", "Token Usage:".bright_white().bold());
        println!("  {} {}", "Input tokens:       ".cyan(), format_number(usage.total_input).bright_white());
        println!("  {} {}", "Output tokens:      ".cyan(), format_number(usage.total_output).bright_white());
        println!("  {} {}", "Cache create tokens:".cyan(), format_number(usage.total_cache_creation).bright_white());
        println!("  {} {}", "Cache read tokens:  ".cyan(), format_number(usage.total_cache_read).bright_white());
        println!("  {} {}", "Total tokens:       ".cyan().bold(), format_number(usage.total()).bright_yellow().bold());
        println!();

        // Stats
        println!("{}", "Statistics:".bright_white().bold());
        println!("  {} {}", "Messages processed: ".cyan(), usage.message_count.to_string().bright_white());
        println!();

        // Estimated cost (assuming Claude Sonnet 4.5 pricing)
        let cost = calculate_cost(&usage);
        println!("{}", "Estimated Cost:".bright_white().bold());
        println!("  {} {}", "Total cost:         ".cyan(), format!("${:.2}", cost).bright_green());
        println!();

        println!("{}", "═".repeat(70).bright_blue());
        println!("  Refreshing every {} seconds... (Ctrl+C to exit)", refresh_seconds);
        println!("{}", "═".repeat(70).bright_blue());

        // Wait before next update
        tokio::time::sleep(Duration::from_secs(refresh_seconds)).await;
    }
}

/// Format number with commas
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

/// Calculate estimated cost based on usage
fn calculate_cost(usage: &local::AggregatedUsage) -> f64 {
    // Claude Sonnet 4.5 pricing (approximation)
    // Input: $3 per million tokens
    // Output: $15 per million tokens
    // Cache write: $3.75 per million tokens
    // Cache read: $0.30 per million tokens

    let input_cost = (usage.total_input as f64 / 1_000_000.0) * 3.0;
    let output_cost = (usage.total_output as f64 / 1_000_000.0) * 15.0;
    let cache_write_cost = (usage.total_cache_creation as f64 / 1_000_000.0) * 3.75;
    let cache_read_cost = (usage.total_cache_read as f64 / 1_000_000.0) * 0.30;

    input_cost + output_cost + cache_write_cost + cache_read_cost
}
