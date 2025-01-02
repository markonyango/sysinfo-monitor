import { Component, effect, input, signal } from '@angular/core';
import { AgGridAngular } from 'ag-grid-angular';
import { GridOptions, GridApi } from 'ag-grid-community';
import { Process } from './types';
import { formatNumber, formatPercent } from '@angular/common';
import { debounceTime, fromEvent } from 'rxjs';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';

@Component({
  standalone: true,
  selector: 'process-list',
  imports: [AgGridAngular],
  template: `
    <ag-grid-angular
      class="ag-theme-material"
      style="height: 600px;"
      [rowData]="data()"
      [gridOptions]="gridOptions()"
    ></ag-grid-angular>
  `,
})
export class ProcessListComponent {
  private gridApi = signal<GridApi | undefined>(undefined);

  public data = input.required<Process[]>();
  
  public gridOptions = signal<GridOptions>({
    columnDefs: [
      {
        field: 'name',
        filter: 'agTextColumnFilter',
      },
      {
        field: 'pid',
        headerName: 'PID',
        sortable: true,
        filter: 'agNumberColumnFilter',
      },
      { field: 'status' },
      { field: 'user_id' },
      {
        field: 'cpu_usage',
        headerName: 'CPU',
        sortable: true,
        sort: 'desc',
        filter: 'agNumberColumnFilter',
        valueFormatter: (params) =>
          `${formatPercent(params.data?.cpu_usage / 100, 'en-EN')}`,
      },
      {
        field: 'memory',
        headerName: 'Memory',
        sortable: true,
        filter: 'agNumberColumnFilter',
        valueFormatter: (params) =>
          `${formatNumber(params.data?.memory / 1024 / 1024, 'en-EN')} MB`,
      },
      // {
      //   field: 'disk_usage',
      //   headerName: 'Disk',
      //   valueFormatter: (params) => {
      //     const read = params.data.disk_usage[0] / 1024 / 1024;
      //     const write = params.data.disk_usage[1] / 1024 / 1024;

      //     return `${formatNumber(read, 'en-EN')} / ${formatNumber(write, 'en-EN')} MB`;
      //   },
      // },
      {
        field: 'run_time',
        headerName: 'Runtime',
        valueFormatter: (params) => {
          const seconds = params.data.run_time;
          const hours = Math.floor(seconds / 3600);
          const minutes = Math.floor((seconds % 3600) / 60);
          const remainingSeconds = seconds % 60;
          return `${hours}h ${minutes}m ${remainingSeconds}s`;
        },
      },
    ],
    onGridReady: (event) => {
      event.api.sizeColumnsToFit();
      this.gridApi.set(event.api);
    },
    onRowDataUpdated(event) {
      event.api.sizeColumnsToFit();
    },
    getRowId: (params) => `${params.data.pid}`,
    animateRows: false,
    suppressCellFocus: true,
    rowSelection: { mode: 'singleRow', checkboxes: false },
  });

  private resizeObserver = fromEvent(window, 'resize').pipe(
    debounceTime(100),
    takeUntilDestroyed(),
  );

  ngOnInit() {
    this.resizeObserver.subscribe(() => this.gridApi()?.sizeColumnsToFit());
  }
}
