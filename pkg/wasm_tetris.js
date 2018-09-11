/* tslint:disable */
import * as wasm from './wasm_tetris_bg';

const __wbg_random_c51ed30e14d59192_target = Math.random;

export function __wbg_random_c51ed30e14d59192() {
    return __wbg_random_c51ed30e14d59192_target();
}

const TextDecoder = typeof self === 'object' && self.TextDecoder
    ? self.TextDecoder
    : require('util').TextDecoder;

let cachedDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
    if (cachedGlobalArgumentPtr === null) {
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    }
    return cachedGlobalArgumentPtr;
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}
/**
* @returns {string}
*/
export function render_frame() {
    const retptr = globalArgumentPtr();
    wasm.render_frame(retptr);
    const mem = getUint32Memory();
    const rustptr = mem[retptr / 4];
    const rustlen = mem[retptr / 4 + 1];
    
    const realRet = getStringFromWasm(rustptr, rustlen).slice();
    wasm.__wbindgen_free(rustptr, rustlen * 1);
    return realRet;
    
}

/**
* @returns {void}
*/
export function update_state() {
    return wasm.update_state();
}

/**
* @returns {void}
*/
export function left_input() {
    return wasm.left_input();
}

/**
* @returns {void}
*/
export function right_input() {
    return wasm.right_input();
}

/**
* @returns {void}
*/
export function left_rotate_input() {
    return wasm.left_rotate_input();
}

/**
* @returns {void}
*/
export function right_rotate_input() {
    return wasm.right_rotate_input();
}

