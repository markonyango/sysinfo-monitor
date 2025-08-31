extern crate serde;

use serde::Deserialize;

use self::serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct Process {
    pub pid: u32,
    pub parent: Option<u32>,
    pub name: String,
    pub user_id: Option<String>,
    pub exe: String,
    pub cmd: Vec<String>,
    pub environ: Vec<String>,
    pub memory: u64,
    pub virtual_memory: u64,
    pub cpu_usage: f32,
    pub disk_usage: DiskUsage,
    pub status: String,
    pub cwd: Option<String>,
    pub root: Option<String>,
    pub start_time: u64,
    pub run_time: u64,
}

#[derive(Debug, Default, Serialize)]
pub struct ProcessStats(pub Vec<Process>);

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DiskUsage {
    pub total_written: u64,
    pub written: u64,
    pub total_read: u64,
    pub read: u64,
}

impl From<sysinfo::DiskUsage> for DiskUsage {
    fn from(value: sysinfo::DiskUsage) -> Self {
        Self {
            total_written: value.total_written_bytes,
            written: value.written_bytes,
            total_read: value.total_read_bytes,
            read: value.read_bytes,
        }
    }
}
