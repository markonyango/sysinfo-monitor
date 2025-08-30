use std::{collections::HashMap, future::Future};

use async_trait::async_trait;
use serde_json::Value;

extern crate sysinfo;

pub mod disk;
pub mod docker;
pub mod network;
pub mod process;
pub mod system;

#[derive(Debug, serde::Serialize)]
#[serde(untagged)]
pub enum MonitorData {
    Disk(disk::DiskStats),
    Docker(docker::DockerStats),
    Network(network::NetworkStats),
    Process(process::ProcessStats),
    System(system::SystemStats),
}

pub trait CollectStats: Send {
    fn collect_stats(&mut self) -> MonitorData;
}

pub trait CollectAsyncStats {
    fn collect_stats(&mut self) -> impl Future<Output = MonitorData>;
}

#[async_trait]
pub trait Monitor {
    async fn report(&mut self) -> serde_json::Value;
}

pub struct MonitorRegistry {
    monitors: Vec<(String, Box<dyn Monitor + 'static + Send + Sync>)>,
}
impl MonitorRegistry {
    pub fn new() -> Self {
        Self {
            monitors: Vec::new(),
        }
    }

    pub fn register<M: Monitor + 'static + Send + Sync>(&mut self, monitor: M, name: &str) {
        self.monitors.push((name.to_string(), Box::new(monitor)));
    }

    pub async fn run(&mut self) -> serde_json::Value {
        let mut reports: Vec<(String, Value)> = Vec::new();
        for (name, monitor) in &mut self.monitors {
            reports.push((name.clone(), monitor.report().await));
        }

        let rs = HashMap::<String, Value>::from_iter(reports);

        serde_json::to_value(rs).unwrap_or_default()
    }
}
