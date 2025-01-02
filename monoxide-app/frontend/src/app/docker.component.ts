import { JsonPipe } from '@angular/common';
import { Component, input, signal } from '@angular/core';
import { AgGridAngular } from 'ag-grid-angular';
import { GridOptions, GridApi } from 'ag-grid-community';
import { DockerInformation } from './types';

@Component({
  standalone: true,
  selector: 'docker-stats',
  imports: [JsonPipe, AgGridAngular],
  template: `<ag-grid-angular
  class="ag-theme-material"
  style="height: 600px;"
  [rowData]="data().containers"
  [gridOptions]="gridOptions()"
></ag-grid-angular>`,
})
export class DockerStatsComponent {
  private gridApi = signal<GridApi | undefined>(undefined);
  
  public data = input.required<DockerInformation>();

  public gridOptions = signal<GridOptions>({
    columnDefs: [
      { field: 'Name' },
      { field: 'Image' },
      { field: 'Created', filter: 'agDateColumnFilter', valueGetter: params => new Date(params.data.Created * 1000), valueFormatter: params => formatDate(params.data.Created) },
      { field: 'Ports', valueFormatter: params => formatPorts(params.data.Ports) },
      { field: 'State', sort: 'desc' },
      { field: 'Status' },
    ],
    onGridReady: (event) => {
      event.api.sizeColumnsToFit();
      this.gridApi.set(event.api);
    },
    onRowDataUpdated(event) {
      event.api.sizeColumnsToFit();
    },
    getRowId: (params) => `${params.data.Id}`,
    animateRows: false,
    suppressCellFocus: true,
    rowSelection: { mode: 'singleRow', checkboxes: false },
  })
}

function formatPorts(ports: any[]) {
  return (ports ?? []).map(port => `${port?.PrivatePort}:${port?.PublicPort}`).join(', ')
}

function formatDate(date: number): string {
  const _date = new Date(date * 1000);
  return Intl.DateTimeFormat('en-US', { year: 'numeric', month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit', second: '2-digit', hour12: false }).format(_date)
}
