import "allocator/arena";

@external("env", "hash160")
declare function hash(): void;

@external("env", "stack_dup")
declare function dup(): void;

@external("env", "hex_decode")
declare function hexDecode(): void;

@external("env", "compare")
declare function compare(): i32;

@external("env", "verify_sig")
declare function verifySig(): i32;

@external("env", "utf16_to_stack")
declare function store(arr: string): void;

const pubKeyHash = "2ef1eacc8cad29a27a54312731d6f3624e013e46";

// var arr = new Array<u16>();
// arr[0] = 5
// arr[1] = 20

dup();
hash();
store(pubKeyHash);
hexDecode();
if (compare() !== 1) {
    unreachable()
}
if (verifySig() !== 1) {
    unreachable()
}
