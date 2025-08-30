use async_trait::async_trait;
use monoxide_backend::{disk::DiskMonitor, process::ProcessMonitor, Monitor, MonitorRegistry};

#[tokio::main]
async fn main() {
    let mut registry = MonitorRegistry::new();

    registry.register(ProcessMonitor::new(), "processes");
    registry.register(DiskMonitor::new(), "disks");
    registry.register(CustomMonitor::new(), "custom");

    let reports = registry.run().await;

    println!("{}", reports.to_string());

}

struct CustomMonitor;
impl CustomMonitor {
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Monitor for CustomMonitor {
    async fn report(&mut self) -> serde_json::Value {
        "Report from CustomMonitor".into()
    }
}
