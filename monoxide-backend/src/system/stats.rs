extern crate serde;

use self::serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct SystemStats {
    pub cpus: Vec<Cpu>,
    pub uptime: u64,
    pub boot_time: u64,
    pub total_memory: u64,
    pub free_memory: u64,
    pub available_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub free_swap: u64,
    pub used_swap: u64,
    pub name: Option<String>,
    pub kernel_version: Option<String>,
    pub os_version: Option<String>,
    pub long_os_version: Option<String>,
    pub distribution_id: String,
    pub host_name: Option<String>,
    pub cpu_arch: String,
    pub physical_core_count: Option<usize>,
}

#[derive(Debug, Default, Serialize)]
pub struct Cpu {
    pub name: String,
    pub vendor_id: String,
    pub usage: f32,
    pub brand: String,
    pub frequency: u64,
}

impl From<&sysinfo::Cpu> for Cpu {
    fn from(value: &sysinfo::Cpu) -> Self {
        Cpu {
            name: value.name().to_string(),
            vendor_id: value.vendor_id().to_string(),
            usage: value.cpu_usage(),
            brand: value.brand().to_string(),
            frequency: value.frequency(),
        }
    }
}
