use std::future::Future;

extern crate sysinfo;

#[cfg(feature = "battery")]
pub mod battery;
pub mod disk;
#[cfg(feature = "docker")]
pub mod docker;
pub mod network;
pub mod process;
pub mod system;

#[derive(Debug, serde::Serialize)]
#[serde(untagged)]
pub enum MonitorData {
    #[cfg(feature = "battery")]
    Battery(battery::BatteryStats),
    Disk(disk::DiskStats),
    #[cfg(feature = "docker")]
    Docker(docker::DockerStats),
    Network(network::NetworkStats),
    Process(process::ProcessStats),
    System(system::SystemStats),
}

pub trait CollectStats {
    fn collect_stats(&mut self) -> MonitorData;
}

pub trait CollectAsyncStats {
    type StatsType;

    fn collect_stats(&mut self) -> impl Future<Output = Result<Self::StatsType, String>>;
}
