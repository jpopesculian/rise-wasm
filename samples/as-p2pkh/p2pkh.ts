import "../../assembly/allocators/arena";

@external("env", "hash160")
declare function hash(input: Uint8Array): Uint8Array;

@external("env", "hex_decode_utf16")
declare function hexDecode(encoded: string): Uint8Array;

@external("env", "compare")
declare function compare(left: Uint8Array, right: Uint8Array): bool;

@external("env", "verify_sig")
declare function verifySig(sig: Uint8Array, pubKey: Uint8Array): bool;

@external("env", "table_load_typed_arr")
declare function loadArg(index: u32): Uint8Array;

const localHash = "2ef1eacc8cad29a27a54312731d6f3624e013e46";

function start(): void {
    let signature = loadArg(0);
    let pubKey = loadArg(1);
    let pubHash = hash(pubKey);
    let localHashArr = hexDecode(localHash);

    if (!compare(localHashArr, pubHash)) {
        throw new Error("Public keys don't match");
    }
    if(!verifySig(signature, pubKey)) {
        throw new Error("Signature doesn't match");
    }
}

start();
