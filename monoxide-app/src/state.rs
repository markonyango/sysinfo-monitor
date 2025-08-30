use monoxide_backend::{
    disk::DiskMonitor, docker::DockerMonitor, network::NetworkMonitor, process::ProcessMonitor,
    system::SystemMonitor, MonitorRegistry,
};
use serde_json::Value;

pub struct AppState {
    monitor_registry: MonitorRegistry,
}

impl AppState {
    pub fn new() -> Self {
        let docker_monitor = DockerMonitor::new().unwrap();
        let process_monitor = ProcessMonitor::new();
        let system_monitor = SystemMonitor::new();
        let network_monitor = NetworkMonitor::new();
        let disk_monitor = DiskMonitor::new();

        let mut monitor_registry = MonitorRegistry::new();

        monitor_registry.register(docker_monitor, "docker");
        monitor_registry.register(process_monitor, "process");
        monitor_registry.register(system_monitor, "system");
        monitor_registry.register(network_monitor, "network");
        monitor_registry.register(disk_monitor, "disk");

        Self { monitor_registry }
    }

    pub async fn update_all(&mut self) -> Value {
        self.monitor_registry.run().await
    }
}
