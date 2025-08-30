use sysinfo::Disks;

use crate::{CollectStats, Monitor, MonitorData};

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

impl Monitor for DiskMonitor {
    fn report(&mut self) -> serde_json::Value {
        self.monitor.refresh(true);

        let info = DiskStats(self.monitor.list().iter().map(Into::into).collect());

        serde_json::to_value(info).unwrap_or_default()
    }
}
