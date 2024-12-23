extern crate sysinfo;

pub mod disk;
pub mod network;
pub mod process;
pub mod system;

pub trait CollectStats {
    type StatsType;

    fn collect_stats(&mut self) -> Self::StatsType;
}
