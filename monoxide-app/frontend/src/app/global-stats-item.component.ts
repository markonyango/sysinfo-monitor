import { ChangeDetectionStrategy, Component, input } from '@angular/core';

@Component({
  standalone: true,
  selector: 'stats-item',
  styles: [
    `
      .stat-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin: 0;
        padding: 0;
        font-size: 0.8rem;
        line-height: 1.2;
      }

      .stat-item span:last-child {
        font-weight: 500;
      }
    `,
  ],
  template: `
    <div class="stat-item">
      <span>{{ label() }}</span>
      <span>{{ value() }}</span>
    </div>
  `,
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class GlobalStatsItemComponent {
  public value = input.required();
  public label = input.required();
}
