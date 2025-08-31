extern crate serde;

use self::serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DiskStats(pub Vec<Disk>);

#[derive(Debug, Default, Serialize)]
pub struct Disk {
    pub name: String,
    pub disk_type: String,
    pub file_system: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
    pub is_removable: bool,
    pub is_readonly: bool,
    pub written_bytes: u64,
    pub read_bytes: u64,
    pub total_written_bytes: u64,
    pub total_read_bytes: u64,
}

impl From<&sysinfo::Disk> for Disk {
    fn from(value: &sysinfo::Disk) -> Self {
        Disk {
            name: value.name().to_string_lossy().to_string(),
            disk_type: value.kind().to_string(),
            file_system: value.file_system().to_string_lossy().to_string(),
            mount_point: value.mount_point().to_string_lossy().to_string(),
            total_space: value.total_space(),
            available_space: value.available_space(),
            is_removable: value.is_removable(),
            is_readonly: value.is_read_only(),
            written_bytes: value.usage().written_bytes,
            read_bytes: value.usage().read_bytes,
            total_written_bytes: value.usage().total_written_bytes,
            total_read_bytes: value.usage().total_read_bytes,
        }
    }
}

