import { DecimalPipe } from '@angular/common';
import { Component, signal } from '@angular/core';

@Component({
  standalone: true,
  imports: [DecimalPipe],
  template: `<span>{{ memory_usage() | number: '0.1-1' }} MB</span>`,
})
export class MemoryUsageRendererComponent {
  public memory_usage = signal(0);

  public agInit(params: any) {
    this.memory_usage.set(params.data.memory_usage / 1024 / 1024);
  }

  public refresh() {
    return true;
  }
}
