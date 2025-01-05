extern crate serde;
use crate::MonitorData;

use self::serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BatteryStats(pub Vec<Battery>);

#[derive(Debug, Default, Serialize)]
pub struct Battery {
    pub cycle_count: Option<u32>,
    pub serial_number: Option<String>,
    pub temperature: Option<f32>,
    pub capacity: Option<f32>,
}

impl From<battery::Battery> for Battery {
    fn from(value: battery::Battery) -> Self {
        Battery {
            cycle_count: value.cycle_count(),
            ..Battery::default()
        }
    }
}

impl From<BatteryStats> for MonitorData {
    fn from(value: BatteryStats) -> Self {
        MonitorData::Battery(value)
    }
}
