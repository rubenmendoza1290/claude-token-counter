use anyhow::{Context, Result};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use crate::models::UsageResponse;

const API_BASE_URL: &str = "https://api.anthropic.com/v1";

pub struct AnthropicClient {
    client: reqwest::Client,
    api_key: String,
}

impl AnthropicClient {
    /// Create a new Anthropic API client with the given API key
    pub fn new(api_key: String) -> Result<Self> {
        let client = reqwest::Client::new();
        Ok(Self { client, api_key })
    }

    /// Build headers for API requests
    fn build_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();

        headers.insert(
            "x-api-key",
            HeaderValue::from_str(&self.api_key)
                .context("Invalid API key format")?,
        );

        headers.insert(
            "anthropic-version",
            HeaderValue::from_static("2023-06-01"),
        );

        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        Ok(headers)
    }

    /// Fetch usage data from the Anthropic API
    ///
    /// Requires an Admin API key (sk-ant-admin-...)
    /// Uses the Claude Code Analytics API endpoint
    /// Note: This endpoint returns data for ONE day at a time
    pub async fn fetch_usage(
        &self,
        days_back: u32,
    ) -> Result<UsageResponse> {
        // Query today's date (since data appears within 5 minutes)
        let target_date = chrono::Utc::now();
        let starting_at = target_date.format("%Y-%m-%d").to_string();

        let url = format!("{}/organizations/usage_report/claude_code", API_BASE_URL);

        let request = self.client
            .get(&url)
            .headers(self.build_headers()?)
            .query(&[
                ("starting_at", starting_at.as_str()),
                ("limit", "1000"), // Max limit to get all records
            ]);

        let response = request
            .send()
            .await
            .context("Failed to send request to Anthropic API")?;

        // Check for HTTP errors
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();

            // Provide helpful error messages
            if status.as_u16() == 404 {
                anyhow::bail!(
                    "API endpoint not found. This likely means:\n\
                     1. You need an Admin API key (starts with 'sk-ant-admin-...')\n\
                     2. Regular API keys (sk-ant-api...) don't have access to usage data\n\
                     3. Get an Admin key from: https://console.anthropic.com/settings/keys\n\n\
                     Error details: {}", body
                );
            }

            anyhow::bail!(
                "API request failed with status {}: {}",
                status,
                body
            );
        }

        // Get response text for debugging
        let response_text = response.text().await?;

        // Print for debugging
        eprintln!("Claude Code API Response:\n{}", response_text);

        // Parse the JSON response
        let usage_response: UsageResponse = serde_json::from_str(&response_text)
            .context("Failed to parse API response as JSON")?;

        Ok(usage_response)
    }
}
