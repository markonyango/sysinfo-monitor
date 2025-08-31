import { Component, input } from '@angular/core';
import { MatIconModule } from '@angular/material/icon';

@Component({
  standalone: true,
  imports: [MatIconModule],
  template: `<mat-icon fontSet="material-icons-outlined" (click)="_action()">{{ icon() }}</mat-icon>`,
})
export class ActionIconComponent {
  public action = input<any>();
  public icon = input.required<string>();
  public params = input.required<any>();

  protected _action() {
    let actionCb = this.action();
    if (actionCb) {
      actionCb(this.params());
    }
  }
}

