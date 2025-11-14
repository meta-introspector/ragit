use ragit_types::api_error::ApiError as Error;
use ragit_model::{Model, ModelRaw, QualityExpectations};
use serde::Deserialize;
use std::fs;
use std::time::{Duration, Instant};

#[derive(Debug, Deserialize)]
struct RateLimits {
    models: Vec<ModelRateLimit>,
}

#[derive(Debug, Deserialize)]
struct ModelRateLimit {
    id: String,
    requests_per_minute: u32,
    requests_per_day: u32,
    tokens_per_minute: u32,
    tokens_per_day: i32,
    quality_expectations: QualityExpectations,
    expected_response_time_ms: u64,
    initial_score: String,
    api_keys: Option<Vec<String>>,
    api_env_vars: Option<Vec<String>>,
}

pub fn merge_rate_limits(models: &mut Vec<ModelRaw>) -> Result<(), Error> {
    let toml_content = match fs::read_to_string("groq_rate_limits.toml") {
        Ok(content) => content,
        Err(_) => return Ok(()), // If the file doesn't exist, just skip merging rate limits
    };

    let rate_limits: RateLimits = match toml::from_str(&toml_content) {
        Ok(limits) => limits,
        Err(_) => return Ok(()), // If the file is invalid, just skip merging rate limits
    };

    for model in models.iter_mut() {
        if let Some(rate_limit) = rate_limits.models.iter().find(|rl| rl.id == model.name) {
            model.requests_per_minute = Some(rate_limit.requests_per_minute);
            model.requests_per_day = Some(rate_limit.requests_per_day);
            model.tokens_per_minute = Some(rate_limit.tokens_per_minute);
            model.tokens_per_day = Some(rate_limit.tokens_per_day);
            model.quality_expectations = Some(rate_limit.quality_expectations.clone());
            model.expected_response_time_ms = Some(rate_limit.expected_response_time_ms);
            model.initial_score = Some(rate_limit.initial_score.clone());
            model.api_keys = rate_limit.api_keys.clone();
            model.api_env_vars = rate_limit.api_env_vars.clone();
        }
    }

    Ok(())
}

pub struct RateLimiter {
    requests_per_minute: u32,
    tokens_per_minute: u32,
    request_tracker: TimeWindowTracker,
    token_tracker: TimeWindowTracker,
    safety_margin: f64,
}

impl RateLimiter {
    pub fn new(model: &Model, safety_margin: f64) -> Self {
        RateLimiter {
            requests_per_minute: model.requests_per_minute.unwrap_or(30),
            tokens_per_minute: model.tokens_per_minute.unwrap_or(6000),
            request_tracker: TimeWindowTracker::new(Duration::from_secs(60)),
            token_tracker: TimeWindowTracker::new(Duration::from_secs(60)),
            safety_margin,
        }
    }

    pub fn check_and_throttle(&mut self) -> Result<Duration, Error> {
        let req_count = self.request_tracker.count();
        let token_count = self.token_tracker.count();

        if (req_count as f64) >= (self.requests_per_minute as f64 * self.safety_margin) {
            return Ok(Duration::from_millis(1000)); // Wait 1s if approaching request limit
        }
        if (token_count as f64) >= (self.tokens_per_minute as f64 * self.safety_margin) {
            return Ok(Duration::from_millis(1000)); // Wait 1s if approaching token limit
        }
        // Calculate minimum delay to stay within limits
        let req_delay = (60_000.0 / self.requests_per_minute as f64) * self.safety_margin;
        Ok(Duration::from_millis(req_delay as u64))
    }

    pub fn record_usage(&mut self, requests: u32, tokens: u32) {
        self.request_tracker.add(requests);
        self.token_tracker.add(tokens);
    }
}

pub struct TimeWindowTracker {
    events: Vec<(Instant, u32)>,
    window: Duration,
}

impl TimeWindowTracker {
    pub fn new(window: Duration) -> Self {
        TimeWindowTracker {
            events: Vec::new(),
            window,
        }
    }

    pub fn add(&mut self, count: u32) {
        self.events.push((Instant::now(), count));
        self.prune();
    }

    pub fn count(&self) -> u32 {
        self.events.iter().map(|(_, c)| c).sum()
    }

    fn prune(&mut self) {
        let now = Instant::now();
        self.events
            .retain(|(time, _)| now.duration_since(*time) <= self.window);
    }
}
