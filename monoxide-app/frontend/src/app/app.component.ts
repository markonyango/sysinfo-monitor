import { Component, computed, inject, Injectable } from '@angular/core';
import { MatCardModule } from '@angular/material/card';
import { MatIconModule } from '@angular/material/icon';
import { MatTabsModule } from '@angular/material/tabs';

import { invoke, InvokeArgs, InvokeOptions } from '@tauri-apps/api/core';

import { DecimalPipe, JsonPipe, NgFor } from '@angular/common';
import { ProcessListComponent } from './process-list.component';
import { InformationService } from './information.service';
import { DiskStatsListComponent } from './disk_stats_list.component';
import { BytesPipe } from './bytes.pipe';
import { NetworkInterfacesListComponent } from './network_interfaces_list.component';
import { GlobalStatsItemComponent } from './global-stats-item.component';
import { GlobalStatsPanelComponent } from './global-stats-panel.component';
import { DockerStatsComponent } from "./docker.component";

@Injectable({ providedIn: 'root' })
export class TauriService {
  public invoke<T = unknown>(
    fn: string,
    args?: InvokeArgs,
    options?: InvokeOptions,
  ) {
    return invoke<T>(fn, args, options);
  }
}

@Component({
  standalone: true,
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css'],
  imports: [
    MatCardModule,
    NgFor,
    BytesPipe,
    GlobalStatsItemComponent,
    GlobalStatsPanelComponent,
    ProcessListComponent,
    DiskStatsListComponent,
    NetworkInterfacesListComponent,
    MatIconModule,
    MatTabsModule,
    DockerStatsComponent
],
})
export class AppComponent {
  private informationService = inject(InformationService);

  public processes = this.informationService.processes;

  public sysinfo = this.informationService.sysinfo;

  public uptime = this.informationService.uptime;

  public cpu_usage = this.informationService.cpu_usage;

  public memory_usage = this.informationService.memory_usage;

  public network_interfaces = this.informationService.network_interfaces;

  public network_global_stats = this.informationService.network_global_stats;

  public disk_usage = this.informationService.disk_usage;

  public disk_global_stats = this.informationService.disk_global_stats;

  public docker_stats = this.informationService.docker_stats;
}
