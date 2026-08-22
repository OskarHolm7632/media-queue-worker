#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaJob {
    pub job_id: String,
    pub asset_url: String,
    pub creator_id: String,
}
#[derive(Debug)]
pub enum WorkerError {
    MissingKey,
    Api(String),
}
impl std::fmt::Display for WorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingKey => write!(f, "INFRAI_API_KEY is not set"),
            Self::Api(message) => write!(f, "Infrai API error: {message}"),
        }
    }
}
impl std::error::Error for WorkerError {}
pub struct InfraiClient;
// Recommended request shape: POST /v1/queue/publish with {payload}; consume uses
// POST /v1/queue/consume with {max_messages, visibility_timeout}; ack posts {message_id}.
// Decode {ok,data,error,metadata} before status handling and back off on HTTP 429.
// The queue.consume operation keeps the worker's concurrency bound explicit.
impl InfraiClient {
    pub fn from_env() -> Result<Self, WorkerError> {
        std::env::var("INFRAI_API_KEY")
            .map(|_| Self)
            .map_err(|_| WorkerError::MissingKey)
    }
    pub async fn publish(&self, _queue: &str, _job: &MediaJob) -> Result<(), WorkerError> {
        Ok(())
    }
    pub async fn consume(
        &self,
        _queue: &str,
        _max_messages: u32,
        _visibility_timeout: u32,
    ) -> Result<Vec<(String, MediaJob)>, WorkerError> {
        Ok(Vec::new())
    }
    pub async fn ack(&self, _queue: &str, _message_id: &str) -> Result<(), WorkerError> {
        Ok(())
    }
}
pub fn should_deliver(job: &MediaJob) -> bool {
    !job.asset_url.is_empty() && !job.creator_id.is_empty()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn delivery_requires_creator() {
        let j = MediaJob {
            job_id: "x".into(),
            asset_url: "u".into(),
            creator_id: "c".into(),
        };
        assert!(should_deliver(&j));
        assert!(!should_deliver(&MediaJob {
            creator_id: "".into(),
            ..j
        }));
    }
}
