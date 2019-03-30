(module
 (type $FUNCSIG$v (func))
 (type $FUNCSIG$vi (func (param i32)))
 (type $FUNCSIG$i (func (result i32)))
 (import "env" "memory" (memory $0 1))
 (data (i32.const 8) "(\00\00\002\00e\00f\001\00e\00a\00c\00c\008\00c\00a\00d\002\009\00a\002\007\00a\005\004\003\001\002\007\003\001\00d\006\00f\003\006\002\004\00e\000\001\003\00e\004\006\00")
 (import "env" "stack_dup" (func $samples/as-p2pkh/p2pkh/dup))
 (import "env" "hash160" (func $samples/as-p2pkh/p2pkh/hash))
 (import "env" "utf16_to_stack" (func $samples/as-p2pkh/p2pkh/store (param i32)))
 (import "env" "hex_decode" (func $samples/as-p2pkh/p2pkh/hexDecode))
 (import "env" "compare" (func $samples/as-p2pkh/p2pkh/compare (result i32)))
 (import "env" "verify_sig" (func $samples/as-p2pkh/p2pkh/verifySig (result i32)))
 (table $0 1 funcref)
 (elem (i32.const 0) $null)
 (global $samples/as-p2pkh/p2pkh/pubKeyHash i32 (i32.const 8))
 (global $~lib/memory/HEAP_BASE i32 (i32.const 92))
 (export "memory" (memory $0))
 (export "table" (table $0))
 (start $start)
 (func $start:samples/as-p2pkh/p2pkh (; 6 ;) (type $FUNCSIG$v)
  call $samples/as-p2pkh/p2pkh/dup
  call $samples/as-p2pkh/p2pkh/hash
  global.get $samples/as-p2pkh/p2pkh/pubKeyHash
  call $samples/as-p2pkh/p2pkh/store
  call $samples/as-p2pkh/p2pkh/hexDecode
  call $samples/as-p2pkh/p2pkh/compare
  i32.const 1
  i32.ne
  if
   unreachable
  end
  call $samples/as-p2pkh/p2pkh/verifySig
  i32.const 1
  i32.ne
  if
   unreachable
  end
 )
 (func $start (; 7 ;) (type $FUNCSIG$v)
  call $start:samples/as-p2pkh/p2pkh
 )
 (func $null (; 8 ;) (type $FUNCSIG$v)
 )
)
