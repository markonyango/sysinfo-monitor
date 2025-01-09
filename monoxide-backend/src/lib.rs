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
