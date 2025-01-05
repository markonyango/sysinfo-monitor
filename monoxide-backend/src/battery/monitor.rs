use crate::{CollectStats, MonitorData};
use battery::Manager;

use super::BatteryStats;

#[derive(Debug)]
pub struct BatteryMonitor {
    pub monitor: Manager,
}

impl BatteryMonitor {
    pub fn new() -> Self {
        let manager = Manager::new().expect("Could not instantiate BatterManager");
        Self { monitor: manager }
    }
}

impl CollectStats for BatteryMonitor {
    fn collect_stats(&mut self) -> MonitorData {
        BatteryStats(
            self.monitor
                .batteries()
                .unwrap()
                .into_iter()
                .flat_map(|battery| battery.ok())
                .map(Into::into)
                .collect(),
        )
        .into()
    }
}
