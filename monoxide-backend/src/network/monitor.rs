use crate::{CollectStats, MonitorData};
use sysinfo::Networks;

use super::{stats::NetworkInterface, NetworkStats};

#[derive(Debug, Default)]
pub struct NetworkMonitor {
    pub monitor: Networks,
}

impl NetworkMonitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl CollectStats for NetworkMonitor {
    fn collect_stats(&mut self) -> MonitorData {
        self.monitor.refresh(true);

        NetworkStats(
            self.monitor
                .list()
                .iter()
                .map(|(key, value)| {
                    let network_interface = NetworkInterface {
                        name: key.to_string(),
                        ..value.into()
                    };

                    network_interface
                })
                .collect(),
        )
        .into()
    }
}
