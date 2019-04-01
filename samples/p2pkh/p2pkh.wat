(module
  (func $hash
    (import "env" "hash160") (param i32) (param i32) (result i32))
  (func $hex_decode
    (import "env" "hex_decode_utf8") (param i32) (param i32) (result i32))
  (func $compare
    (import "env" "compare") (param i32) (param i32) (result i32))
  (func $verify_sig
    (import "env" "verify_sig") (param i32) (param i32) (result i32))
  (func $load_arr
    (import "env" "table_load_typed_arr") (param i32) (param i32) (result i32))

  (import "env" "memory" (memory 1))
  (data (i32.const 0) "\28\00\00\002ef1eacc8cad29a27a54312731d6f3624e013e46")

  (func $main
    (local $sig_ptr i32)
    (local $pubkey_ptr i32)
    (local $pubhash_ptr i32)
    (local $localhash_ptr i32)
    (set_local $sig_ptr (i32.const 512))
    (set_local $pubkey_ptr (i32.const 1024))
    (set_local $pubhash_ptr (i32.const 1536))
    (set_local $localhash_ptr (i32.const 0))

    (drop (call $load_arr (i32.const 0) (get_local $sig_ptr)))
    (drop (call $load_arr (i32.const 1) (get_local $pubkey_ptr)))

    (drop (call $hash (get_local $pubkey_ptr) (get_local $pubhash_ptr)))
    (drop (call $hex_decode (get_local $localhash_ptr) (get_local $localhash_ptr)))
    (if (call $compare (get_local $localhash_ptr) (get_local $pubhash_ptr))
        (then
            (if (call $verify_sig (get_local $sig_ptr) (get_local $pubkey_ptr))
              (then (nop))
              (else (unreachable))))
        (else
            (unreachable)))
    )
  (start $main)
  )
