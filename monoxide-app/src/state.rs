use std::collections::HashMap;

use monoxide_backend::{
    disk::{DiskMonitor, DiskStats},
    docker::{DockerMonitor, DockerStats},
    network::{NetworkMonitor, NetworkStats},
    process::{ProcessMonitor, ProcessStats},
    system::{SystemMonitor, SystemStats},
    CollectAsyncStats, CollectStats,
};
use serde::Serialize;
use tauri::async_runtime::Mutex;

pub type Results = HashMap<String, MonitorStats>;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum MonitorStats {
    Disk(DiskStats),
    Docker(DockerStats),
    Network(NetworkStats),
    Process(ProcessStats),
    System(SystemStats),
}

#[derive(Debug)]
pub struct AppState {
    pub process_monitor: Mutex<ProcessMonitor>,
    pub system_monitor: Mutex<SystemMonitor>,
    pub network_monitor: Mutex<NetworkMonitor>,
    pub disk_monitor: Mutex<DiskMonitor>,
    pub docker_monitor: Mutex<DockerMonitor>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            docker_monitor: Mutex::new(DockerMonitor::new()),
            process_monitor: Mutex::new(ProcessMonitor::new()),
            system_monitor: Mutex::new(SystemMonitor::new()),
            network_monitor: Mutex::new(NetworkMonitor::new()),
            disk_monitor: Mutex::new(DiskMonitor::new()),
        }
    }

    pub async fn update_all(&self) -> Result<HashMap<String, MonitorStats>, String> {
        let mut map = HashMap::new();

        map.insert(
            "disk".into(),
            MonitorStats::Disk(self.disk_monitor.lock().await.collect_stats()),
        );

        map.insert(
            "network".into(),
            MonitorStats::Network(self.network_monitor.lock().await.collect_stats()),
        );
        map.insert(
            "system".into(),
            MonitorStats::System(self.system_monitor.lock().await.collect_stats()),
        );
        map.insert(
            "process".into(),
            MonitorStats::Process(self.process_monitor.lock().await.collect_stats()),
        );
        map.insert(
            "docker".into(),
            MonitorStats::Docker(
                self.docker_monitor
                    .lock()
                    .await
                    .collect_stats()
                    .await
                    .unwrap(),
            ),
        );

        Ok(map)
    }

    pub async fn update_disks(&self) -> Result<DiskStats, String> {
        let mut monitor = self.disk_monitor.lock().await;
        Ok(monitor.collect_stats())
    }

    pub async fn update_network(&self) -> Result<NetworkStats, String> {
        let mut monitor = self.network_monitor.lock().await;
        Ok(monitor.collect_stats())
    }

    pub async fn update_system(&self) -> Result<SystemStats, String> {
        let mut monitor = self.system_monitor.lock().await;
        Ok(monitor.collect_stats())
    }

    pub async fn update_processes(&self) -> Result<ProcessStats, String> {
        let mut monitor = self.process_monitor.lock().await;
        Ok(monitor.collect_stats())
    }

    pub async fn update_docker_stats(&self) -> Result<DockerStats, String> {
        let mut monitor = self.docker_monitor.lock().await;
        Ok(monitor.collect_stats().await.unwrap())
    }
}
