import { formatNumber } from '@angular/common';

export function formatBytes(bytes: number): string {
  let output: [number, number, number, number, number] = [
    bytes,
    bytes / 1024,
    bytes / 1024 / 1024,
    bytes / 1024 / 1024 / 1024,
    bytes / 1024 / 1024 / 1024 / 1024,
  ];

  if (output[0] < 1024) {
    return `${formatNumber(output[0], 'en-EN')} b`;
  } else if (output[1] < 1024) {
    return `${formatNumber(output[1], 'en-EN', '0.2-2')} Kb`;
  } else if (output[2] < 1024) {
    return `${formatNumber(output[2], 'en-EN', '0.2-2')} Mb`;
  } else if (output[3] < 1024) {
    return `${formatNumber(output[3], 'en-EN', '0.2-2')} Gb`;
  } else {
    return `${formatNumber(output[4], 'en-EN', '0.2-2')} Tb`;
  }
}
