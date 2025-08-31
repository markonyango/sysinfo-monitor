use crate::Monitor;
use async_trait::async_trait;
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

#[async_trait]
impl Monitor for NetworkMonitor {
    async fn report(&mut self) -> serde_json::Value {
        self.monitor.refresh(true);

        let info = NetworkStats(
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
        );

        serde_json::to_value(info).unwrap_or_default()
    }
}
