use monoxide_backend::{docker::DockerMonitor, CollectAsyncStats};

#[tokio::main]
async fn main() {
    let mut monitor = DockerMonitor::new();
    println!("{:?}", monitor.collect_stats().await);
}
