use monoxide_backend::{disk::DiskMonitor, process::ProcessMonitor, Monitor, MonitorRegistry};

#[tokio::main]
async fn main() {
    let mut registry = MonitorRegistry::new();

    registry.register(ProcessMonitor::new());
    registry.register(DiskMonitor::new());
    registry.register(CustomMonitor::new());

    let reports = registry.run();

    println!("{}", reports.to_string());

}

struct CustomMonitor;
impl CustomMonitor {
    fn new() -> Self {
        Self
    }
}
impl Monitor for CustomMonitor {
    fn report(&mut self) -> serde_json::Value {
        "Report from CustomMonitor".into()
    }
}
