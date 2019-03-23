(module
  (func $hash
    (import "imports" "hash160"))
  (func $compare
    (import "imports" "compare") (result i32))
  (func $verify_sig
    (import "imports" "verify_sig") (param i32) (param i32) (result i32))
  (func $add
    (import "imports" "add") (param i32) (param i32) (result i32))
  (func $load
    (import "imports" "hex_mem_to_stack") (param i32) (param i32))

  (import "memory" "default" (memory 1))
  (data (i32.const 0) "2ef1eacc8cad29a27a54312731d6f3624e013e46")

  (func $add_40 (param i32) (result i32)
    (call $add (i32.const 40) (i32.const 0)))

  (func (export "main") (result i32) (local $hash_out i32)
    (call $hash) ;; hash public key (arg[1])
    (call $load (i32.const 0) (i32.const 40)) ;; add hashed public key from memory to stack
    (if (result i32)
        (call $compare) ;; compare hashes
            (then
                (call $verify_sig (i32.const 0) (i32.const 128))) ;; verify signature
            (else
                (i32.const 0)))
    ))
