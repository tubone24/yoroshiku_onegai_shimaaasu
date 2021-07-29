/* tslint:disable */
/* eslint-disable */
/**
* @param {string} e_str
* @param {string} c_str
* @param {string} n_str
* @param {string} cheat
*/
export function calc(e_str: string, c_str: string, n_str: string, cheat: string): void;
/**
* @param {string} e_str
* @param {string} n_str
* @param {string} plain_text
*/
export function create_crypt_num(e_str: string, n_str: string, plain_text: string): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly calc: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
  readonly create_crypt_num: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
