import { Pipe, PipeTransform } from '@angular/core';
import { formatBytes } from './utils';

@Pipe({ standalone: true, name: 'bytes', pure: true })
export class BytesPipe implements PipeTransform {
  transform(bytes: number) {
    return formatBytes(bytes);
  }
}
