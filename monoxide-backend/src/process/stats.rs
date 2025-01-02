extern crate serde;

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
    pub status: String,
    pub cwd: Option<String>,
    pub root: Option<String>,
    pub start_time: u64,
    pub run_time: u64,
}

#[derive(Debug, Default, Serialize)]
pub struct ProcessStats(pub Vec<Process>);
