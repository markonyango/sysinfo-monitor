import { computed, effect, inject, Injectable } from '@angular/core';
import { toSignal } from '@angular/core/rxjs-interop';
import { concatMap, interval, tap } from 'rxjs';
import { DiskInformation, Information, Process, SysInfo } from './types';
import { TauriService } from './app.component';

const initialState: Information = {
  disk: [],
  network: [],
  system: {
    cpus: [],
    uptime: 0,
    boot_time: 0,
    total_memory: 0,
    free_memory: 0,
    available_memory: 0,
    used_memory: 0,
    total_swap: 0,
    free_swap: 0,
    used_swap: 0,
    name: '',
    kernel_version: '',
    os_version: '',
    long_os_version: '',
    distribution_id: '',
    host_name: '',
    cpu_arch: '',
    physical_core_count: 0,
  },
  process: [],
  docker: { images: [], containers: [] }
};

@Injectable({ providedIn: 'root' })
export class InformationService {
  private tauriService = inject(TauriService);

  private data = toSignal(
    interval(1500).pipe(
      concatMap(() => this.tauriService.invoke<Information>('update_all')),
    ),
    { initialValue: initialState },
  );

  public processes = computed<Process[]>(() => this.data().process);

  public sysinfo = computed<SysInfo>(() => this.data().system);

  public cpu_usage = computed(() => this.sysinfo()?.cpus);

  public network_interfaces = computed(() => this.data().network ?? []);

  public network_global_stats = computed(() => ({
    receiving: this.network_interfaces()
      .map((network_interface) => network_interface.received)
      .reduce((a, b) => a + b, 0),
    transmitting: this.network_interfaces()
      .map((network_interface) => network_interface.transmitted)
      .reduce((a, b) => a + b, 0),
  }));

  public disk_usage = computed<DiskInformation[]>(() => this.data().disk ?? []);

  public disk_global_stats = computed(() => ({
    total: this.disk_usage()
      .map((disk) => disk.total_space)
      .reduce((a, b) => a + b, 0),
    used: this.disk_usage()
      .map((disk) => disk.total_space - disk.available_space)
      .reduce((a, b) => a + b, 0),
    free: this.disk_usage()
      .map((disk) => disk.available_space)
      .reduce((a, b) => a + b, 0),
  }));

  public memory_usage = computed(() => ({
    usage: calculateTotalMemoryUsage(
      this.sysinfo().used_memory,
      this.sysinfo().total_memory,
    ),
    total: this.sysinfo()?.total_memory,
    free: this.sysinfo()?.free_memory,
    used: this.sysinfo()?.used_memory,
    cached: 0, // FIXME: Add this stat asap
  }));

  public uptime = computed(() => {
    const seconds = this.sysinfo().uptime;
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const remainingSeconds = seconds % 60;
    return `${hours}h ${minutes}m ${remainingSeconds}s`;
  });

  public docker_stats = computed(() => this.data().docker);
}

function calculateTotalMemoryUsage(
  memory_used: number,
  memory_total: number,
): string {
  if (memory_total == 0) {
    return '0';
  }

  return Intl.NumberFormat('en-EN', { style: 'percent' }).format((memory_used / memory_total));
}
