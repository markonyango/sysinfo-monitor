import { DecimalPipe } from '@angular/common';
import { Component, signal } from '@angular/core';

@Component({
  standalone: true,
  imports: [DecimalPipe],
  template: `<span>{{ cpu_usage | number: '0.1-1' }}%</span>`,
})
export class CPUUsageRendererComponent {
  public cpu_usage = 0;

  public agInit(params: any) {
    this.cpu_usage = params.data.cpu_usage;
  }

  public refresh() {
    return true;
  }
}
