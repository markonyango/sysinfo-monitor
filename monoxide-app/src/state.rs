use std::collections::HashMap;

use monoxide_backend::{
    disk::DiskMonitor, docker::DockerMonitor, network::NetworkMonitor, process::ProcessMonitor,
    system::SystemMonitor, CollectAsyncStats, CollectStats, MonitorData,
};

pub type Results = HashMap<String, MonitorData>;

pub struct AppState {
    pub process_monitor: ProcessMonitor,
    pub system_monitor: SystemMonitor,
    pub network_monitor: NetworkMonitor,
    pub disk_monitor: DiskMonitor,
    pub docker_monitor: DockerMonitor,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            docker_monitor: DockerMonitor::new(),
            process_monitor: ProcessMonitor::new(),
            system_monitor: SystemMonitor::new(),
            network_monitor: NetworkMonitor::new(),
            disk_monitor: DiskMonitor::new(),
        }
    }

    pub async fn update_all(&mut self) -> Result<HashMap<String, MonitorData>, String> {
        let mut map = HashMap::new();

        map.insert("disk".into(), self.disk_monitor.collect_stats().into());

        map.insert(
            "network".into(),
            self.network_monitor.collect_stats().into(),
        );
        map.insert("system".into(), self.system_monitor.collect_stats().into());
        map.insert(
            "process".into(),
            self.process_monitor.collect_stats().into(),
        );
        map.insert(
            "docker".into(),
            self.docker_monitor.collect_stats().await.into(),
        );

        Ok(map)
    }

    pub fn update_disks(&mut self) -> MonitorData {
        self.disk_monitor.collect_stats()
    }

    pub fn update_network(&mut self) -> MonitorData {
        self.network_monitor.collect_stats()
    }

    pub fn update_system(&mut self) -> MonitorData {
        self.system_monitor.collect_stats()
    }

    pub fn update_processes(&mut self) -> MonitorData {
        self.process_monitor.collect_stats()
    }

    pub async fn update_docker_stats(&mut self) -> MonitorData {
        self.docker_monitor.collect_stats().await
    }
}
