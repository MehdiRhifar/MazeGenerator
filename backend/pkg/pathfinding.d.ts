/* tslint:disable */
/* eslint-disable */

export enum AlgorithmKind {
  Backtracking = 0,
  Prim = 1,
  Kruskal = 2,
  Wilson = 3,
  RecursiveDivision = 4,
}

export class MazeGenerator {
  free(): void;
  [Symbol.dispose](): void;
  clear_grid(): void;
  resize_grid(new_width: number, new_height: number): void;
  generate_maze(algorithm: AlgorithmKind): void;
  get_grid_width(): number;
  generation_step(): boolean;
  get_cell_layers(): Array<any>;
  get_grid_height(): number;
  start_generation(algorithm: AlgorithmKind): void;
  has_vertical_wall(x: number, y: number): boolean;
  has_horizontal_wall(x: number, y: number): boolean;
  generation_step_with_changes(): object;
  constructor(width: number, height: number);
  fill_grid(): void;
}

export class Point {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  x: number;
  y: number;
}

export class WallChange {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  x: number;
  y: number;
  wall_type: WallType;
}

export enum WallType {
  Vertical = 0,
  Horizontal = 1,
}

export function greet(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_get_point_x: (a: number) => number;
  readonly __wbg_get_point_y: (a: number) => number;
  readonly __wbg_mazegenerator_free: (a: number, b: number) => void;
  readonly __wbg_point_free: (a: number, b: number) => void;
  readonly __wbg_set_point_x: (a: number, b: number) => void;
  readonly __wbg_set_point_y: (a: number, b: number) => void;
  readonly greet: () => void;
  readonly mazegenerator_clear_grid: (a: number) => void;
  readonly mazegenerator_fill_grid: (a: number) => void;
  readonly mazegenerator_generate_maze: (a: number, b: number) => void;
  readonly mazegenerator_generation_step: (a: number) => number;
  readonly mazegenerator_generation_step_with_changes: (a: number) => any;
  readonly mazegenerator_get_cell_layers: (a: number) => any;
  readonly mazegenerator_get_grid_height: (a: number) => number;
  readonly mazegenerator_get_grid_width: (a: number) => number;
  readonly mazegenerator_has_horizontal_wall: (a: number, b: number, c: number) => number;
  readonly mazegenerator_has_vertical_wall: (a: number, b: number, c: number) => number;
  readonly mazegenerator_new: (a: number, b: number) => number;
  readonly mazegenerator_resize_grid: (a: number, b: number, c: number) => void;
  readonly mazegenerator_start_generation: (a: number, b: number) => void;
  readonly __wbg_get_wallchange_wall_type: (a: number) => number;
  readonly __wbg_get_wallchange_x: (a: number) => number;
  readonly __wbg_get_wallchange_y: (a: number) => number;
  readonly __wbg_set_wallchange_wall_type: (a: number, b: number) => void;
  readonly __wbg_set_wallchange_x: (a: number, b: number) => void;
  readonly __wbg_set_wallchange_y: (a: number, b: number) => void;
  readonly __wbg_wallchange_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
