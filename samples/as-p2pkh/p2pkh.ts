import "../../assembly/allocators/arena";

@external("env", "hash160")
declare function hash(src: Uint8Array, dest: Uint8Array): u32;

@external("env", "hex_decode_utf16")
declare function hexDecode(src: string, dest: Uint8Array): u32;

@external("env", "compare")
declare function compare(left: Uint8Array, right: Uint8Array): bool;

@external("env", "verify_sig")
declare function verifySig(sig: Uint8Array, pubKey: Uint8Array): bool;

@external("env", "table_load_typed_arr")
declare function loadArray(index: u32, dest: Uint8Array): u32;

const localHash = "2ef1eacc8cad29a27a54312731d6f3624e013e46";

function start(): void {
    let signature: Uint8Array = changetype<Uint8Array>(memory.allocate(256));
    let pubKey: Uint8Array = changetype<Uint8Array>(memory.allocate(256));
    let pubHash: Uint8Array = changetype<Uint8Array>(memory.allocate(256));
    let localHashArr: Uint8Array = changetype<Uint8Array>(memory.allocate(256));

    loadArray(0, signature);
    loadArray(1, pubKey);
    hash(pubKey, pubHash);
    hexDecode(localHash, localHashArr);

    if (!compare(localHashArr, pubHash)) {
        unreachable();
    }
    if(!verifySig(signature, pubKey)) {
        unreachable();
    }
}

start();
