use serde::{Deserialize, Serialize};

/// Response from the Anthropic Usage & Cost API
#[derive(Debug, Deserialize, Serialize)]
pub struct UsageResponse {
    /// Usage data grouped by time buckets
    pub data: Vec<UsageRecord>,

    /// Whether there are more pages
    #[serde(default)]
    pub has_more: bool,

    /// Next page token for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
}

/// Individual usage record for a time bucket
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UsageRecord {
    /// Start time of this bucket (ISO 8601)
    pub starting_at: String,

    /// End time of this bucket (ISO 8601)
    pub ending_at: String,

    /// Results array containing usage details
    #[serde(default)]
    pub results: Vec<UsageDetail>,
}

/// Detailed usage information within a time bucket
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UsageDetail {
    /// Input tokens (uncached)
    #[serde(default)]
    pub input_tokens: u64,

    /// Output tokens
    #[serde(default)]
    pub output_tokens: u64,

    /// Cache creation tokens
    #[serde(default)]
    pub cache_creation_input_tokens: u64,

    /// Cache read tokens
    #[serde(default)]
    pub cache_read_input_tokens: u64,
}

impl UsageRecord {
    /// Calculate total tokens (input + output + cache) for this bucket
    pub fn total(&self) -> u64 {
        self.results.iter().map(|r| r.total()).sum()
    }

    /// Get input tokens for this bucket
    pub fn input_tokens(&self) -> u64 {
        self.results.iter().map(|r| r.input_tokens).sum()
    }

    /// Get output tokens for this bucket
    pub fn output_tokens(&self) -> u64 {
        self.results.iter().map(|r| r.output_tokens).sum()
    }

    /// Get date string from starting_at
    pub fn date(&self) -> String {
        // Extract just the date part (YYYY-MM-DD) from ISO 8601 timestamp
        self.starting_at.split('T').next().unwrap_or(&self.starting_at).to_string()
    }
}

impl UsageDetail {
    /// Calculate total tokens (input + output + cache)
    pub fn total(&self) -> u64 {
        self.input_tokens + self.output_tokens + self.cache_creation_input_tokens + self.cache_read_input_tokens
    }
}

/// Summary of total usage across all records
#[derive(Debug)]
pub struct UsageSummary {
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub total_tokens: u64,
    pub days_with_usage: usize,
}

impl UsageSummary {
    /// Create a summary from a list of usage records
    pub fn from_records(records: &[UsageRecord]) -> Self {
        let total_input_tokens: u64 = records.iter().map(|r| r.input_tokens()).sum();
        let total_output_tokens: u64 = records.iter().map(|r| r.output_tokens()).sum();
        let total_tokens = total_input_tokens + total_output_tokens;
        let days_with_usage = records.iter().filter(|r| r.total() > 0).count();

        Self {
            total_input_tokens,
            total_output_tokens,
            total_tokens,
            days_with_usage,
        }
    }

    /// Calculate percentage used of a given limit
    pub fn percentage_used(&self, limit: u64) -> f64 {
        if limit == 0 {
            return 0.0;
        }
        (self.total_tokens as f64 / limit as f64) * 100.0
    }

    /// Calculate remaining tokens from a limit
    pub fn remaining(&self, limit: u64) -> i64 {
        limit as i64 - self.total_tokens as i64
    }
}
