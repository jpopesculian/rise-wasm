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

@external("env", "typed_arr_to_stack")
declare function store(arr: Uint16Array, elem_size: u32): void;

const pubKeyHash = "2ef1eacc8cad29a27a54312731d6f3624e013e46";

var arr: Uint16Array = new Uint16Array(11);
arr[0] = 1
arr[1] = 2
arr[2] = 3

dup();
hash();
store(arr, 2);
hexDecode();
if (compare() !== 1) {
    unreachable()
}
if (verifySig() !== 1) {
    unreachable()
}
