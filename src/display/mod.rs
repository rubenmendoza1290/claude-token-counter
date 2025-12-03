use crate::models::{UsageSummary, UsageRecord};
use colored::*;

/// Display the status with colored output
pub fn display_status(summary: &UsageSummary, monthly_limit: Option<u64>) {
    println!("\n{}", "═".repeat(60).bright_blue());
    println!("{}", "  TOKEN USAGE SUMMARY".bright_cyan().bold());
    println!("{}", "═".repeat(60).bright_blue());

    // Display token counts
    println!("\n{}", "Token Counts:".bright_white().bold());
    println!("  {} {}", "Input tokens: ".cyan(), format_number(summary.total_input_tokens).bright_white());
    println!("  {} {}", "Output tokens:".cyan(), format_number(summary.total_output_tokens).bright_white());
    println!("  {} {}", "Total tokens: ".cyan().bold(), format_number(summary.total_tokens).bright_yellow().bold());

    // Display usage stats
    println!("\n{}", "Usage Stats:".bright_white().bold());
    println!("  {} {}", "Days with usage:".cyan(), summary.days_with_usage.to_string().bright_white());

    // If monthly limit is provided, show progress
    if let Some(limit) = monthly_limit {
        let percentage = summary.percentage_used(limit);
        let remaining = summary.remaining(limit);

        println!("\n{}", "Monthly Quota:".bright_white().bold());
        println!("  {} {}", "Limit:       ".cyan(), format_number(limit).bright_white());
        println!("  {} {}", "Used:        ".cyan(), format_number(summary.total_tokens).bright_yellow());

        if remaining >= 0 {
            println!("  {} {}", "Remaining:   ".cyan(), format_number(remaining as u64).bright_green());
        } else {
            println!("  {} {}", "Overage:     ".cyan(), format_number(remaining.abs() as u64).bright_red());
        }

        // Display percentage with color coding
        let percentage_display = format!("{:.1}%", percentage);
        let colored_percentage = if percentage < 50.0 {
            percentage_display.green()
        } else if percentage < 80.0 {
            percentage_display.yellow()
        } else if percentage < 100.0 {
            percentage_display.bright_yellow()
        } else {
            percentage_display.red().bold()
        };

        println!("  {} {}", "Usage:       ".cyan(), colored_percentage);

        // Display progress bar
        display_progress_bar(percentage);
    }

    println!("\n{}", "═".repeat(60).bright_blue());
}

/// Display a progress bar for usage percentage
fn display_progress_bar(percentage: f64) {
    let bar_width = 40;
    let filled = ((percentage / 100.0) * bar_width as f64) as usize;
    let filled = filled.min(bar_width);

    let mut bar = String::from("  [");

    for i in 0..bar_width {
        if i < filled {
            bar.push('█');
        } else {
            bar.push('░');
        }
    }

    bar.push(']');

    // Color the bar based on usage
    let colored_bar = if percentage < 50.0 {
        bar.green()
    } else if percentage < 80.0 {
        bar.yellow()
    } else if percentage < 100.0 {
        bar.bright_yellow()
    } else {
        bar.red().bold()
    };

    println!("{}", colored_bar);
}

/// Format a number with thousand separators
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

/// Display history of usage over time
pub fn display_history(records: &[UsageRecord], days: u32) {
    println!("\n{}", "═".repeat(80).bright_blue());
    println!("{}", format!("  USAGE HISTORY - Last {} Days", days).bright_cyan().bold());
    println!("{}", "═".repeat(80).bright_blue());

    if records.is_empty() {
        println!("\n  {}", "No usage data found for the specified period.".yellow());
        println!("\n{}", "═".repeat(80).bright_blue());
        return;
    }

    // Header
    println!("\n  {:<12} {:>15} {:>15} {:>15}",
        "Date".cyan().bold(),
        "Input".cyan().bold(),
        "Output".cyan().bold(),
        "Total".cyan().bold()
    );
    println!("  {}", "─".repeat(76).bright_black());

    // Sort records by date and display
    let mut sorted_records = records.to_vec();
    sorted_records.sort_by(|a, b| b.date().cmp(&a.date()));

    for record in sorted_records.iter().take(days as usize) {
        let total = record.total();

        // Color code based on usage
        let total_colored = if total > 100_000 {
            format_number(total).red()
        } else if total > 50_000 {
            format_number(total).yellow()
        } else {
            format_number(total).white()
        };

        println!("  {:<12} {:>15} {:>15} {:>15}",
            record.date().bright_white(),
            format_number(record.input_tokens()).white(),
            format_number(record.output_tokens()).white(),
            total_colored
        );
    }

    println!("\n{}", "═".repeat(80).bright_blue());
}
