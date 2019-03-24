(module
  (func $hash
    (import "imports" "hash160"))
  (func $dup
    (import "imports" "stack_dup"))
  (func $hex_decode
    (import "imports" "hex_decode"))
  (func $compare
    (import "imports" "compare") (result i32))
  (func $verify_sig
    (import "imports" "verify_sig") (result i32))
  (func $load
    (import "imports" "mem_to_stack") (param i32) (param i32))

  (import "memory" "default" (memory 1))
  (data (i32.const 0) "2ef1eacc8cad29a27a54312731d6f3624e013e46")

  (func (export "main") (result i32) (local $hash_out i32)
    ;; stack: [sig, pubKey]
    (call $dup) ;; duplicate top stack arg
    ;; stack: [sig, pubKey, pubKey]
    (call $hash) ;; hash public key
    ;; stack: [sig, pubKey, hashedPubKey]
    (call $load (i32.const 0) (i32.const 40)) ;; add hashed public key from memory to stack
    ;; stack: [sig, pubKey, hashedPubKey, scriptMemHex]
    (call $hex_decode) ;; decode hex string to bytes
    ;; stack: [sig, pubKey, hashedPubKey, scriptMem]
    (if (result i32)
        (call $compare) ;; compare hashes
            (then
                ;; stack: [sig, pubKey]
                (call $verify_sig)) ;; verify signature
            (else
                (i32.const 0)))
    ))
