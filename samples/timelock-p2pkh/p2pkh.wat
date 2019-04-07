(module
 (type $FUNCSIG$vi (func (param i32)))
 (type $FUNCSIG$v (func))
 (type $FUNCSIG$jiiiiii (func (param i32 i32 i32 i32 i32 i32) (result i64)))
 (type $FUNCSIG$ii (func (param i32) (result i32)))
 (type $FUNCSIG$iii (func (param i32 i32) (result i32)))
 (type $FUNCSIG$viiii (func (param i32 i32 i32 i32)))
 (import "env" "memory" (memory $0 1))
 (data (i32.const 8) "(\00\00\002\00e\00f\001\00e\00a\00c\00c\008\00c\00a\00d\002\009\00a\002\007\00a\005\004\003\001\002\007\003\001\00d\006\00f\003\006\002\004\00e\000\001\003\00e\004\006\00")
 (data (i32.const 96) "\1f\00\00\00s\00a\00m\00p\00l\00e\00s\00/\00t\00i\00m\00e\00l\00o\00c\00k\00-\00p\002\00p\00k\00h\00/\00p\002\00p\00k\00h\00.\00t\00s\00")
 (import "env" "now" (global $assembly/helpers/time/NOW i64))
 (import "env" "mem_init_buddy" (func $assembly/allocators/buddy/mem_init (param i32)))
 (import "env" "epoch_time" (func $assembly/helpers/time/epochTime (param i32 i32 i32 i32 i32 i32) (result i64)))
 (import "env" "table_load_typed_arr" (func $assembly/helpers/arrays/loadTypedArray<~lib/typedarray/Uint8Array> (param i32) (result i32)))
 (import "env" "hash160" (func $assembly/helpers/crypto/hash160 (param i32) (result i32)))
 (import "env" "hex_decode_utf16" (func $assembly/helpers/strings/hexDecode (param i32) (result i32)))
 (import "env" "compare" (func $assembly/helpers/arrays/compare (param i32 i32) (result i32)))
 (import "env" "abort" (func $~lib/env/abort (param i32 i32 i32 i32)))
 (import "env" "verify_sig" (func $assembly/helpers/crypto/verifySig (param i32 i32) (result i32)))
 (table $0 1 funcref)
 (elem (i32.const 0) $null)
 (global $samples/timelock-p2pkh/p2pkh/localHash i32 (i32.const 8))
 (global $~lib/memory/HEAP_BASE i32 (i32.const 164))
 (export "memory" (memory $0))
 (export "table" (table $0))
 (start $start)
 (func $start:assembly/allocators/buddy (; 8 ;) (type $FUNCSIG$v)
  global.get $~lib/memory/HEAP_BASE
  call $assembly/allocators/buddy/mem_init
 )
 (func $samples/timelock-p2pkh/p2pkh/checkSig (; 9 ;) (type $FUNCSIG$v)
  (local $0 i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  block $assembly/helpers/arrays/loadUint8Array|inlined.0 (result i32)
   i32.const 0
   local.set $0
   local.get $0
   call $assembly/helpers/arrays/loadTypedArray<~lib/typedarray/Uint8Array>
  end
  local.set $0
  block $assembly/helpers/arrays/loadUint8Array|inlined.1 (result i32)
   i32.const 1
   local.set $1
   local.get $1
   call $assembly/helpers/arrays/loadTypedArray<~lib/typedarray/Uint8Array>
  end
  local.set $1
  local.get $1
  call $assembly/helpers/crypto/hash160
  local.set $2
  global.get $samples/timelock-p2pkh/p2pkh/localHash
  call $assembly/helpers/strings/hexDecode
  local.set $3
  local.get $3
  local.get $2
  call $assembly/helpers/arrays/compare
  i32.const 0
  i32.ne
  i32.eqz
  if
   i32.const 0
   i32.const 96
   i32.const 21
   i32.const 4
   call $~lib/env/abort
   unreachable
  end
  local.get $0
  local.get $1
  call $assembly/helpers/crypto/verifySig
  i32.const 0
  i32.ne
  i32.eqz
  if
   i32.const 0
   i32.const 96
   i32.const 24
   i32.const 4
   call $~lib/env/abort
   unreachable
  end
 )
 (func $samples/timelock-p2pkh/p2pkh/start (; 10 ;) (type $FUNCSIG$v)
  global.get $assembly/helpers/time/NOW
  i32.const 2019
  i32.const 3
  i32.const 1
  i32.const 0
  i32.const 0
  i32.const 0
  call $assembly/helpers/time/epochTime
  i64.gt_s
  if
   call $samples/timelock-p2pkh/p2pkh/checkSig
  else   
   i32.const 0
   i32.const 96
   i32.const 32
   i32.const 4
   call $~lib/env/abort
   unreachable
  end
 )
 (func $start:samples/timelock-p2pkh/p2pkh (; 11 ;) (type $FUNCSIG$v)
  call $start:assembly/allocators/buddy
  call $samples/timelock-p2pkh/p2pkh/start
 )
 (func $start (; 12 ;) (type $FUNCSIG$v)
  call $start:samples/timelock-p2pkh/p2pkh
 )
 (func $null (; 13 ;) (type $FUNCSIG$v)
 )
)
