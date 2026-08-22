use media_queue_worker::{should_deliver, InfraiClient, MediaJob};
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = InfraiClient::from_env()?;
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    let queue = format!("codecheck-media-worker-{suffix}");
    let job = MediaJob {
        job_id: format!("demo-asset-{suffix}"),
        asset_url: "https://cdn.example/media.mp4".into(),
        creator_id: "creator-7".into(),
    };
    client.publish(&queue, &job).await?;
    for (message_id, job) in client.consume(&queue, 10, 30).await? {
        if should_deliver(&job) {
            println!("deliver {} to {}", job.job_id, job.creator_id);
        }
        client.ack(&queue, &message_id).await?;
    }
    Ok(())
}
