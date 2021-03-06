(module
  (func $mem_init
    (import "env" "mem_init_arena") (param i32))
  (func $hash
    (import "env" "hash160") (param i32) (result i32))
  (func $hex_decode
    (import "env" "hex_decode_utf8") (param i32) (result i32))
  (func $compare
    (import "env" "compare") (param i32) (param i32) (result i32))
  (func $verify_sig
    (import "env" "verify_sig") (param i32) (param i32) (result i32))
  (func $load_arr
    (import "env" "table_load_typed_arr") (param i32) (result i32))

  (import "env" "memory" (memory 1))
  (data (i32.const 0) "\28\00\00\002ef1eacc8cad29a27a54312731d6f3624e013e46")

  (func $main
    (local $signature i32)
    (local $pubkey i32)
    (local $pubhash i32)
    (local $localhash i32)

    (call $mem_init (i32.const 64))
    (set_local $localhash (i32.const 0))

    (set_local $signature (call $load_arr (i32.const 0)))
    (set_local $pubkey (call $load_arr (i32.const 1)))

    (set_local $pubhash (call $hash (get_local $pubkey)))
    (set_local $localhash (call $hex_decode (get_local $localhash)))

    (if (call $compare (get_local $localhash) (get_local $pubhash))
        (then
            (if (call $verify_sig (get_local $signature) (get_local $pubkey))
              (then (nop))
              (else (unreachable))))
        (else
            (unreachable)))
    )
  (start $main)
  )
