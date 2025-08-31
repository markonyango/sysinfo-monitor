import { NgComponentOutlet, NgForOf } from '@angular/common';
import { Component, InputSignal, Type } from '@angular/core';

import { IconAction } from './icon-action';
import { ICellRendererAngularComp } from 'ag-grid-angular';
import { ICellRendererParams } from 'ag-grid-community';

interface ActionIconComponent { action: InputSignal<(...args: unknown[]) => unknown>, params: InputSignal<ICellRendererParams> };
interface IconComponent { component: Type<ActionIconComponent>, action: IconAction, icon?: string };
interface Icons { components: IconComponent[] }

@Component({
  standalone: true,
  styles: [`
    .container {
      display: flex;
      gap: 1em;
      width: 100%
    }
  `],
  template: `
      <div class="container">
        <ng-container *ngFor="let iconComponent of params?.components">
          <ng-container *ngComponentOutlet="iconComponent.component; inputs: { action: iconComponent.action, icon: iconComponent.icon, params }"></ng-container>
        </ng-container>
      </div>
  `,
  imports: [NgForOf, NgComponentOutlet]
})
export class ActionsCellRendererComponent implements ICellRendererAngularComp {

  public params: ICellRendererParams & Icons | undefined;

  agInit(params: ICellRendererParams & Icons): void {
    this.params = params;
  }

  refresh(): boolean {
    return true;
  }
}
