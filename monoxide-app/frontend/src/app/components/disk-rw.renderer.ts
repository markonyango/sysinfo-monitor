import { DecimalPipe } from '@angular/common';
import { Component, computed, signal } from '@angular/core';

@Component({
  standalone: true,
  imports: [DecimalPipe],
  template: `<span
    >{{ read() | number: '0.1-1' }} / {{ write() | number: '0.1-1' }} MB</span
  >`,
})
export class DiskRWRendererComponent {
  private disk_rw = signal([0, 0]);

  public read = computed(() => (this.disk_rw().at(0) ?? 0) / 1024 / 1024);
  public write = computed(() => (this.disk_rw().at(1) ?? 0) / 1024 / 1024);

  public agInit(params: any) {
    this.disk_rw.set(params.data.disk_usage);
  }

  public refresh() {
    return true;
  }
}
