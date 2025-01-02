export type Process = {
  pid: number,
  parent?: number,
  name: string,
  user_id?: string,
  exe: string,
  cmd: string[],
  environ: string[],
  memory: number,
  virtual_memory: number,
  cpu_usage: number,
  status: string,
  cwd?: string,
  root?: string,
  start_time: number,
  run_time: number,
};

type CPU = {
  name: string,
  vendor_id: string,
  usage: number,
  brand: string,
  frequency: number
}

export type SysInfo = {
  cpus: CPU[],
  uptime: number,
  boot_time: number,
  total_memory: number,
  free_memory: number,
  available_memory: number,
  used_memory: number,
  total_swap: number,
  free_swap: number,
  used_swap: number,
  name?: string,
  kernel_version?: string,
  os_version?: string,
  long_os_version?: string,
  distribution_id: String,
  host_name?: string,
  cpu_arch: String,
  physical_core_count?: number,
};

export type NetworkInfo = {
  rx: number;
  tx: number;
  name: String;
  received: number;
  transmitted: number;
  total_received: number;
  total_transmitted: number;
};

export type DiskInformation = {
  name: string,
  file_system: string,
  disk_type: string,
  mount_point: string,
  total_space: number,
  available_space: number,
  is_removable: boolean,
  is_readonly: boolean,
  written_bytes: number,
  read_bytes: number,
  total_written_bytes: number,
  total_read_bytes: number,
};

export type DockerInformation = {
  containers: Container[],
  images: Image[]
}

type Container = {
  Id: string,
  Name: string[],
  Image: string,
  ImageID: string,
  Command: string,
  Created: number,
  Ports: { IP: string, PrivatePort: number, PublicPort: number, Type: string }[],
  SizeRw: number,
  SizeRootFs: number,
  Labels: Record<string, string>,
  State: string,
  Status: string,
}

type Image = {}

export type Information = { disk: DiskInformation[], network: any[], system: any, process: Process[], docker: DockerInformation };
