use crate::{CollectStats, Monitor, MonitorData};
use async_trait::async_trait;
use sysinfo::System;

use super::stats::SystemStats;

#[derive(Debug, Default)]
pub struct SystemMonitor {
    pub monitor: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl CollectStats for SystemMonitor {
    fn collect_stats(&mut self) -> MonitorData {
        self.monitor.refresh_all();

        SystemStats {
            cpus: self.monitor.cpus().iter().map(Into::into).collect(),
            uptime: System::uptime(),
            boot_time: System::boot_time(),
            total_memory: self.monitor.total_memory(),
            free_memory: self.monitor.free_memory(),
            available_memory: self.monitor.available_memory(),
            used_memory: self.monitor.used_memory(),
            total_swap: self.monitor.total_swap(),
            free_swap: self.monitor.free_swap(),
            used_swap: self.monitor.used_swap(),
            name: System::name(),
            kernel_version: System::kernel_version(),
            os_version: System::os_version(),
            long_os_version: System::long_os_version(),
            distribution_id: System::distribution_id(),
            host_name: System::host_name(),
            cpu_arch: System::cpu_arch(),
            physical_core_count: sysinfo::System::physical_core_count(),
        }
        .into()
    }
}

#[async_trait]
impl Monitor for SystemMonitor {
    async fn report(&mut self) -> serde_json::Value {
        self.monitor.refresh_all();

        let info = SystemStats {
            cpus: self.monitor.cpus().iter().map(Into::into).collect(),
            uptime: System::uptime(),
            boot_time: System::boot_time(),
            total_memory: self.monitor.total_memory(),
            free_memory: self.monitor.free_memory(),
            available_memory: self.monitor.available_memory(),
            used_memory: self.monitor.used_memory(),
            total_swap: self.monitor.total_swap(),
            free_swap: self.monitor.free_swap(),
            used_swap: self.monitor.used_swap(),
            name: System::name(),
            kernel_version: System::kernel_version(),
            os_version: System::os_version(),
            long_os_version: System::long_os_version(),
            distribution_id: System::distribution_id(),
            host_name: System::host_name(),
            cpu_arch: System::cpu_arch(),
            physical_core_count: sysinfo::System::physical_core_count(),
        };

        serde_json::to_value(info).unwrap_or_default()
    }
}
