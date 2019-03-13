(module
  (func $hash
    (import "imports" "hash160") (param i32) (param i32))
  (func $compare
    (import "imports" "compare") (param i32) (param i32) (param i32) (result i32))
  (func $verify_sig
    (import "imports" "verify_sig") (param i32) (param i32) (result i32))

  (global $s
    (import "globals" "start_index") i32)

  (import "memory" "default" (memory 1))
  (data (get_global $s) "2ef1eacc8cad29a27a54312731d6f3624e013e46")

  (func (export "main") (result i32) (local $hash_out i32)
    (set_local $hash_out (i32.add (i32.const 40) (get_global $s)))
    (call $hash (i32.const 128) (get_local $hash_out)) ;; hash public key
    (if (result i32)
        (call $compare (i32.const 194) (get_local $hash_out) (i32.const 40)) ;; compare hashes
            (then
                (call $verify_sig (i32.const 0) (i32.const 128))) ;; verify signature
            (else
                (i32.const 0)))
    ))
