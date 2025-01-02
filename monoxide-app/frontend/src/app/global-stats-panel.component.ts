import { ChangeDetectionStrategy, Component } from '@angular/core';

@Component({
  standalone: true,
  selector: 'stats-panel',
  styles: [
    `
      .stats-panel {
        display: flex;
        flex-direction: column;
        gap: 8px;

        min-width: 125px;
        padding: 12px;
      }
    `,
  ],
  template: `
    <div class="stats-panel">
      <ng-content />
    </div>
  `,
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class GlobalStatsPanelComponent {}
