type CellRendererParams = Record<string, unknown>;

export type IconAction<T = unknown, R = unknown> = (value: T) => R;


export function cellRendererParamsWithIconAction<T, R>(iconAction: IconAction<T, R>, params?: CellRendererParams) {
  return {
    iconAction,
    ...params,
  }
}

