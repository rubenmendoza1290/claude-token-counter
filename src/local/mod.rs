use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use walkdir::WalkDir;

/// Claude Code JSONL log entry
#[derive(Debug, Deserialize, Serialize)]
pub struct LogEntry {
    #[serde(default)]
    pub message: Option<Message>,
    pub timestamp: Option<String>,
    #[serde(rename = "agentId")]
    pub agent_id: Option<String>,
}

/// Message structure from Claude Code logs
#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub model: Option<String>,
    pub usage: Option<Usage>,
}

/// Token usage information
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Usage {
    #[serde(default)]
    pub input_tokens: u64,
    #[serde(default)]
    pub output_tokens: u64,
    #[serde(default)]
    pub cache_creation_input_tokens: u64,
    #[serde(default)]
    pub cache_read_input_tokens: u64,
}

impl Usage {
    pub fn total(&self) -> u64 {
        self.input_tokens + self.output_tokens + self.cache_creation_input_tokens + self.cache_read_input_tokens
    }
}

/// Aggregated usage statistics
#[derive(Debug, Default, Clone)]
pub struct AggregatedUsage {
    pub total_input: u64,
    pub total_output: u64,
    pub total_cache_creation: u64,
    pub total_cache_read: u64,
    pub message_count: usize,
}

impl AggregatedUsage {
    pub fn total(&self) -> u64 {
        self.total_input + self.total_output + self.total_cache_creation + self.total_cache_read
    }

    pub fn add(&mut self, usage: &Usage) {
        self.total_input += usage.input_tokens;
        self.total_output += usage.output_tokens;
        self.total_cache_creation += usage.cache_creation_input_tokens;
        self.total_cache_read += usage.cache_read_input_tokens;
        self.message_count += 1;
    }
}

/// Find Claude Code projects directory
pub fn get_claude_projects_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    let claude_dir = home.join(".claude").join("projects");

    if !claude_dir.exists() {
        anyhow::bail!(
            "Claude Code projects directory not found at {:?}\n\
             Make sure you have used Claude Code at least once.",
            claude_dir
        );
    }

    Ok(claude_dir)
}

/// Find all JSONL files in Claude Code projects
pub fn find_jsonl_files() -> Result<Vec<PathBuf>> {
    let projects_dir = get_claude_projects_dir()?;

    let mut files = Vec::new();
    for entry in WalkDir::new(projects_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
            files.push(path.to_path_buf());
        }
    }

    Ok(files)
}

/// Parse a single JSONL file and aggregate usage
pub fn parse_jsonl_file(path: &PathBuf) -> Result<AggregatedUsage> {
    let file = File::open(path)
        .with_context(|| format!("Failed to open {:?}", path))?;
    let reader = BufReader::new(file);

    let mut aggregated = AggregatedUsage::default();

    for (line_num, line) in reader.lines().enumerate() {
        let line = line.context("Failed to read line")?;

        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // Parse JSON
        match serde_json::from_str::<LogEntry>(&line) {
            Ok(entry) => {
                if let Some(message) = entry.message {
                    if let Some(usage) = message.usage {
                        aggregated.add(&usage);
                    }
                }
            }
            Err(e) => {
                // Skip malformed lines but warn
                eprintln!("Warning: Failed to parse line {} in {:?}: {}", line_num + 1, path, e);
            }
        }
    }

    Ok(aggregated)
}

/// Parse all JSONL files and return aggregated usage
pub fn parse_all_files() -> Result<AggregatedUsage> {
    let files = find_jsonl_files()?;

    if files.is_empty() {
        anyhow::bail!("No JSONL files found in Claude Code projects directory");
    }

    let mut total = AggregatedUsage::default();

    for file in files {
        match parse_jsonl_file(&file) {
            Ok(usage) => {
                total.total_input += usage.total_input;
                total.total_output += usage.total_output;
                total.total_cache_creation += usage.total_cache_creation;
                total.total_cache_read += usage.total_cache_read;
                total.message_count += usage.message_count;
            }
            Err(e) => {
                eprintln!("Warning: Failed to parse {:?}: {}", file, e);
            }
        }
    }

    Ok(total)
}
