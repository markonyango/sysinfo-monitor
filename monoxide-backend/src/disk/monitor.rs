use sysinfo::Disks;

use crate::{CollectStats, MonitorData};

use super::DiskStats;

#[derive(Debug, Default)]
pub struct DiskMonitor {
    pub monitor: Disks,
}

impl DiskMonitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl CollectStats for DiskMonitor {
    fn collect_stats(&mut self) -> MonitorData {
        self.monitor.refresh(true);

        DiskStats(self.monitor.list().iter().map(Into::into).collect()).into()
    }
}
