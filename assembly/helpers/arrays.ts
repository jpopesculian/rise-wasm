@external("env", "table_load_typed_arr")
export declare function loadTypedArray<T>(index: u32): T;

@external("env", "table_load_arr")
export declare function loadArray<T>(index: u32): Array<T>;

@external("env", "table_store_typed_arr")
export declare function storeTypedArray<T>(index: u32, arr: T, elem_size: u32): void;

@external("env", "table_store_arr")
export declare function storeArray<T>(index: u32, arr: Array<T>): void;

@external("env", "compare")
export declare function compare(left: Uint8Array, right: Uint8Array): bool;

@inline
export function loadUint8Array(index: u32): Uint8Array {
    return loadTypedArray<Uint8Array>(index);
}

@inline
export function storeUint8Array(index: u32, arr: Uint8Array): void {
    return storeTypedArray<Uint8Array>(index, arr, 1);
}
