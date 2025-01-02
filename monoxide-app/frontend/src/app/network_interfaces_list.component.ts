import { Component, HostListener, input, signal } from '@angular/core';
import { AgGridAngular } from 'ag-grid-angular';
import { GridApi, GridOptions } from 'ag-grid-community';
import { formatBytes } from './utils';
import { debounceTime, fromEvent } from 'rxjs';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';

@Component({
  standalone: true,
  selector: 'network-interface-list',
  imports: [AgGridAngular],
  template: `
    <ag-grid-angular
      class="ag-theme-material"
      [rowData]="data()"
      [gridOptions]="gridOptions()"
    ></ag-grid-angular>
  `,
})
export class NetworkInterfacesListComponent {
  private gridApi = signal<GridApi | undefined>(undefined);

  public data = input.required<any>();

  public gridOptions = signal<GridOptions>({
    domLayout: 'autoHeight',
    columnDefs: [
      {
        field: 'name',
      },
      {
        field: 'received',
        valueFormatter: (params) => formatBytes(params.data.received),
      },
      {
        field: 'transmitted',
        valueFormatter: (params) => formatBytes(params.data.transmitted),
      },
      {
        field: 'total_received',
        valueFormatter: (params) => formatBytes(params.data.total_received),
      },
      {
        field: 'total_transmitted',
        valueFormatter: (params) => formatBytes(params.data.total_transmitted),
      },
    ],
    onGridReady: (event) => {
      event.api.sizeColumnsToFit();
      this.gridApi.set(event.api);
    },
    onRowDataUpdated(event) {
      event.api.sizeColumnsToFit();
    },
  });

  private resizeObserver = fromEvent(window, 'resize').pipe(
    debounceTime(100),
    takeUntilDestroyed(),
  );

  ngOnInit() {
    this.resizeObserver.subscribe(() => this.gridApi()?.sizeColumnsToFit());
  }
}
