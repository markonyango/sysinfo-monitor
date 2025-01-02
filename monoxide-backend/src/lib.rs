use std::future::Future;

extern crate sysinfo;

pub mod disk;
#[cfg(feature = "docker")]
pub mod docker;
pub mod network;
pub mod process;
pub mod system;

pub trait CollectStats {
    type StatsType;

    fn collect_stats(&mut self) -> Self::StatsType;
}

pub trait CollectAsyncStats {
    type StatsType;

    fn collect_stats(&mut self) -> impl Future<Output = Result<Self::StatsType, String>>;
}
