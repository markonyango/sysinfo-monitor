import { NgFor } from '@angular/common';
import { Component, HostListener, input, signal } from '@angular/core';
import { GridApi, GridOptions } from 'ag-grid-community';
import { BytesPipe } from './bytes.pipe';

import { AgGridAngular } from 'ag-grid-angular';
import { formatBytes } from './utils';
import { debounceTime, fromEvent } from 'rxjs';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';

@Component({
  standalone: true,
  selector: 'disk-list',
  imports: [AgGridAngular, NgFor, BytesPipe],
  template: `
    <ag-grid-angular
      class="ag-theme-material"
      style="height: 100%;"
      [rowData]="data()"
      [gridOptions]="gridOptions()"
    ></ag-grid-angular>
  `,
})
export class DiskStatsListComponent {
  private gridApi = signal<GridApi | undefined>(undefined);

  public data = input.required<any>();

  public gridOptions = signal<GridOptions>({
    domLayout: 'autoHeight',
    columnDefs: [
      {
        field: 'name',
        headerName: 'Name',
      },
      {
        field: 'file_system',
        headerName: 'File system',
      },
      {
        field: 'mount_point',
        headerName: 'Mount point',
        sort: 'asc'
      },
      {
        field: 'disk_type',
        headerName: 'Type',
      },
      {
        headerName: 'Space (Available / Total)',
        valueFormatter: (params) => `${formatBytes(params.data?.available_space)} / ${formatBytes(params.data?.total_space)}`,
      },
      // {
      //   field: 'available_space',
      //   headerName: 'Available space',
      //   valueFormatter: (params) => formatBytes(params.data?.available_space),
      // },
      // {
      //   field: 'total_space',
      //   headerName: 'Total space',
      //   valueFormatter: (params) => formatBytes(params.data?.total_space),
      // },
      {
        field: 'written_bytes',
        headerName: 'Bytes written',
        valueFormatter: (params) =>
          formatBytes(params.data?.written_bytes),
      },
      {
        field: 'read_bytes',
        headerName: 'Bytes read',
        valueFormatter: (params) => formatBytes(params.data?.read_bytes),
      },
      {
        field: 'total_written_bytes',
        headerName: 'Total Bytes written',
        valueFormatter: (params) =>
          formatBytes(params.data?.total_written_bytes),
      },
      {
        field: 'total_read_bytes',
        headerName: 'Total Bytes read',
        valueFormatter: (params) =>
          formatBytes(params.data?.total_read_bytes),
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
