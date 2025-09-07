/* tslint:disable */
/* eslint-disable */
/**
* Solve constraints directly without creating a solver instance
* @param {string} json_str
* @returns {string}
*/
export function solve_constraints(json_str: string): string;
/**
* Validate a constraint document
* @param {string} json_str
* @returns {boolean}
*/
export function validate_document(json_str: string): boolean;
/**
* Get the constraint system JSON schema
* @returns {string}
*/
export function get_constraint_schema(): string;
/**
* Initialize the WASM module (called automatically)
*/
export function init(): void;

/**
 * SLVSX Constraint Solver WASM Module
 * 
 * Example usage:
 * ```typescript
 * import init, { WasmSolver, solve_constraints } from '@slvsx/core';
 * 
 * // Initialize WASM module
 * await init();
 * 
 * // Option 1: Use solver instance
 * const solver = new WasmSolver();
 * const result = solver.solve(constraintJson);
 * 
 * // Option 2: Use direct function
 * const result = solve_constraints(constraintJson);
 * ```
 */
export interface ConstraintDocument {
  schema: "slvs-json/1";
  units?: string;
  parameters?: Record<string, number>;
  entities: Entity[];
  constraints: Constraint[];
}

export interface SolveResult {
  status: string;
  diagnostics?: {
    iters: number;
    residual: number;
    dof: number;
    time_ms: number;
  };
  entities?: Record<string, ResolvedEntity>;
  warnings: string[];
}


/**
*/
export class WasmSolver {
  free(): void;
/**
* Create a new WASM solver instance
*/
  constructor();
/**
* Solve a constraint system from JSON string
* 
* # Arguments
* * `json_str` - JSON string containing the constraint specification
* 
* # Returns
* JSON string containing the solve result or error
* @param {string} json_str
* @returns {string}
*/
  solve(json_str: string): string;
/**
* Validate a constraint document without solving
* @param {string} json_str
* @returns {boolean}
*/
  validate(json_str: string): boolean;
/**
* Get the JSON schema for constraint documents
* @returns {string}
*/
  static get_schema(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_wasmsolver_free: (a: number) => void;
  readonly wasmsolver_new: () => number;
  readonly wasmsolver_solve: (a: number, b: number, c: number, d: number) => void;
  readonly wasmsolver_validate: (a: number, b: number, c: number, d: number) => void;
  readonly solve_constraints: (a: number, b: number, c: number) => void;
  readonly validate_document: (a: number, b: number, c: number) => void;
  readonly get_constraint_schema: (a: number) => void;
  readonly init: () => void;
  readonly wasmsolver_get_schema: (a: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_export_0: (a: number, b: number) => number;
  readonly __wbindgen_export_1: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
