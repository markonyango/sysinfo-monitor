import { Component, signal } from '@angular/core';

@Component({
  standalone: true,
  imports: [],
  template: `<span>{{ run_time() }}</span>`,
})
export class RuntimeRendererComponent {
  public run_time = signal('');

  public agInit(params: any) {
    this.run_time.set(formatRunTime(params.data.run_time));
  }

  public refresh() {
    return true;
  }
}

/**
 * @see https://github.com/Abdenasser/neohtop/blob/a5ce84c88d50f1ac068cc53d8af3e81c76ff1eeb/src/lib/definitions/columns.ts#L53C5-L59C7
 */
function formatRunTime(seconds: number): string {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const remainingSeconds = seconds % 60;
  return `${hours}h ${minutes}m ${remainingSeconds}s`;
}
