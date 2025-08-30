use crate::{CollectStats, Monitor, MonitorData};
use async_trait::async_trait;
use sysinfo::{ProcessRefreshKind, System};

use super::{stats::Process, ProcessStats};

#[derive(Debug, Default)]
pub struct ProcessMonitor {
    pub monitor: System,
}

impl ProcessMonitor {
    pub fn new() -> Self {
        Self {
            monitor: System::new_all(),
        }
    }
}

impl CollectStats for ProcessMonitor {
    fn collect_stats(&mut self) -> MonitorData {
        let process_refresh_kind = ProcessRefreshKind::nothing()
            .with_memory()
            .with_cmd(sysinfo::UpdateKind::OnlyIfNotSet)
            .with_environ(sysinfo::UpdateKind::OnlyIfNotSet)
            .with_exe(sysinfo::UpdateKind::OnlyIfNotSet)
            .with_user(sysinfo::UpdateKind::OnlyIfNotSet)
            .without_tasks()
            .without_disk_usage()
            .with_cpu();

        self.monitor.refresh_processes_specifics(
            sysinfo::ProcessesToUpdate::All,
            true,
            process_refresh_kind,
        );

        ProcessStats(
            self.monitor
                .processes()
                .iter()
                .map(|(pid, process)| Process {
                    pid: pid.as_u32(),
                    parent: process.parent().map(|p| p.as_u32()),
                    name: process.name().to_string_lossy().to_string(),
                    cmd: process
                        .cmd()
                        .iter()
                        .filter_map(|cmd| cmd.to_owned().into_string().ok())
                        .collect(),
                    environ: process
                        .environ()
                        .iter()
                        .filter_map(|env| env.to_owned().into_string().ok())
                        .collect(),
                    memory: process.memory(),
                    virtual_memory: process.virtual_memory(),
                    cpu_usage: process.cpu_usage(),
                    exe: process
                        .exe()
                        .map_or("".into(), |exe| exe.to_string_lossy().to_string()),
                    user_id: process.user_id().map(|id| id.to_string()),
                    status: process.status().to_string(),
                    cwd: process.cwd().map(|cwd| cwd.to_string_lossy().to_string()),
                    root: process
                        .root()
                        .map(|root| root.to_string_lossy().to_string()),
                    start_time: process.start_time(),
                    run_time: process.run_time(),
                })
                .collect(),
        )
        .into()
    }
}

#[async_trait]
impl Monitor for ProcessMonitor {
    async fn report(&mut self) -> serde_json::Value {
        let process_refresh_kind = ProcessRefreshKind::nothing()
            .with_memory()
            .with_cmd(sysinfo::UpdateKind::OnlyIfNotSet)
            .with_environ(sysinfo::UpdateKind::OnlyIfNotSet)
            .with_exe(sysinfo::UpdateKind::OnlyIfNotSet)
            .with_user(sysinfo::UpdateKind::OnlyIfNotSet)
            .without_tasks()
            .without_disk_usage()
            .with_cpu();

        self.monitor.refresh_processes_specifics(
            sysinfo::ProcessesToUpdate::All,
            true,
            process_refresh_kind,
        );

        let info = ProcessStats(
            self.monitor
                .processes()
                .iter()
                .map(|(pid, process)| Process {
                    pid: pid.as_u32(),
                    parent: process.parent().map(|p| p.as_u32()),
                    name: process.name().to_string_lossy().to_string(),
                    cmd: process
                        .cmd()
                        .iter()
                        .filter_map(|cmd| cmd.to_owned().into_string().ok())
                        .collect(),
                    environ: process
                        .environ()
                        .iter()
                        .filter_map(|env| env.to_owned().into_string().ok())
                        .collect(),
                    memory: process.memory(),
                    virtual_memory: process.virtual_memory(),
                    cpu_usage: process.cpu_usage(),
                    exe: process
                        .exe()
                        .map_or("".into(), |exe| exe.to_string_lossy().to_string()),
                    user_id: process.user_id().map(|id| id.to_string()),
                    status: process.status().to_string(),
                    cwd: process.cwd().map(|cwd| cwd.to_string_lossy().to_string()),
                    root: process
                        .root()
                        .map(|root| root.to_string_lossy().to_string()),
                    start_time: process.start_time(),
                    run_time: process.run_time(),
                })
                .collect(),
        );

        serde_json::to_value(info).unwrap_or_default()
    }
}
