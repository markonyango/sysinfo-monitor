use std::future::Future;

extern crate sysinfo;

pub mod disk;
#[cfg(feature = "docker")]
pub mod docker;
pub mod network;
pub mod process;
pub mod system;

#[derive(Debug, serde::Serialize)]
#[serde(untagged)]
pub enum MonitorData {
    Disk(disk::DiskStats),
    #[cfg(feature = "docker")]
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

pub trait Monitor {
    fn report(&mut self) -> serde_json::Value;
}

pub struct MonitorRegistry {
    monitors: Vec<Box<dyn Monitor + 'static>>,
}
impl MonitorRegistry {
    pub fn new() -> Self {
        Self {
            monitors: Vec::new(),
        }
    }

    pub fn register<M: Monitor + 'static>(&mut self, monitor: M) {
        self.monitors.push(Box::new(monitor));
    }

    pub fn run(&mut self) -> serde_json::Value {
        let rs = self
            .monitors
            .iter_mut()
            .map(|monitor| monitor.report())
            .collect::<Vec<_>>();

        serde_json::to_value(rs).unwrap_or_default()
    }
}
