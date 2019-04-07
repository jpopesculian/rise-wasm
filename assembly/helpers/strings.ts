@external("env", "table_load_utf16")
export declare function loadString(index: u32): string;

@external("env", "table_store_utf16")
export declare function storeString(index: u32, str: string): void;

@external("env", "hex_decode_utf16")
export declare function hexDecode(encoded: string): Uint8Array;
