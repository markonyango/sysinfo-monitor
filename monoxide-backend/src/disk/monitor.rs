use async_trait::async_trait;
use sysinfo::Disks;

use crate::Monitor;

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

#[async_trait]
impl Monitor for DiskMonitor {
    async fn report(&mut self) -> serde_json::Value {
        self.monitor.refresh(true);

        let info = DiskStats(self.monitor.list().iter().map(Into::into).collect());

        serde_json::to_value(info).unwrap_or_default()
    }
}
