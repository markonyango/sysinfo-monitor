use monoxide_backend::{docker::DockerMonitor, CollectAsyncStats};

#[tokio::main]
async fn main() {
    let mut monitor = DockerMonitor::new();
    let _ = monitor.collect_stats().await.and_then(|val| {
        println!("{:#?}", val);
        Ok(val)
    });
}
