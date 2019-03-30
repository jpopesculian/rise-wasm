(module
  (func $hash
    (import "env" "hash160"))
  (func $dup
    (import "env" "stack_dup"))
  (func $hex_decode
    (import "env" "hex_decode"))
  (func $compare
    (import "env" "compare") (result i32))
  (func $verify_sig
    (import "env" "verify_sig") (result i32))
  (func $store
    (import "env" "utf8_to_stack") (param i32))

  (import "env" "memory" (memory 1))
  (data (i32.const 0) "\28\00\00\002ef1eacc8cad29a27a54312731d6f3624e013e46")

  (func $main
    ;; stack: [sig, pubKey]
    (call $dup) ;; duplicate top stack arg
    ;; stack: [sig, pubKey, pubKey]
    (call $hash) ;; hash public key
    ;; stack: [sig, pubKey, hashedPubKey]
    (call $store (i32.const 0)) ;; add hashed public key from memory to stack
    ;; stack: [sig, pubKey, hashedPubKey, memHashedPubKeyHex]
    (call $hex_decode) ;; decode hex string to bytes
    ;; stack: [sig, pubKey, hashedPubKey, memHashedPubKey]
    (if (call $compare) ;; compare hashes
        (then
            ;; stack: [sig, pubKey]
            (if (call $verify_sig) ;; verify signature
              (then (nop)) ;; return none
              (else (unreachable)))) ;; exception
        (else
            (unreachable))) ;; exception
    )
  (start $main)
  )
