use clap::{Parser, Subcommand};
use anyhow::Result;

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
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Status => {
            println!("Fetching current token usage...");
            // TODO: Implement status command
        }
        Commands::History { days } => {
            println!("Showing usage history for the last {} days...", days);
            // TODO: Implement history command
        }
        Commands::Config { api_key } => {
            if let Some(key) = api_key {
                println!("Configuring API key...");
                // TODO: Implement config command
            } else {
                println!("No API key provided");
            }
        }
    }

    Ok(())
}
