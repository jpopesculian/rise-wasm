(module
 (type $FUNCSIG$vi (func (param i32)))
 (type $FUNCSIG$v (func))
 (type $FUNCSIG$ii (func (param i32) (result i32)))
 (type $FUNCSIG$iii (func (param i32 i32) (result i32)))
 (type $FUNCSIG$viiii (func (param i32 i32 i32 i32)))
 (type $FUNCSIG$viii (func (param i32 i32 i32)))
 (import "env" "memory" (memory $0 1))
 (data (i32.const 8) "\0d\00\00\00~\00l\00i\00b\00/\00a\00r\00r\00a\00y\00.\00t\00s\00")
 (data (i32.const 40) "\1c\00\00\00~\00l\00i\00b\00/\00i\00n\00t\00e\00r\00n\00a\00l\00/\00a\00r\00r\00a\00y\00b\00u\00f\00f\00e\00r\00.\00t\00s\00")
 (data (i32.const 104) "B\00\00\000\002\00c\008\001\007\005\00e\008\000\006\009\004\000\00e\008\005\005\001\00c\008\009\002\009\005\006\007\009\00c\00c\001\001\007\00b\004\00e\007\00d\00a\00b\003\00f\002\003\000\005\002\005\004\003\003\004\000\00c\00f\004\001\004\004\003\004\00b\00b\001\00c\009\00")
 (data (i32.const 240) "B\00\00\000\002\00a\008\000\001\00e\00a\00b\008\002\00a\008\00c\004\00f\001\005\006\00e\00d\00b\00a\00d\009\00c\003\00a\00a\000\005\00b\005\00b\007\004\001\004\00d\000\00c\008\007\00f\006\00b\009\008\00a\006\00f\000\00b\002\006\00a\00c\001\004\00d\00d\009\002\009\005\00f\00")
 (data (i32.const 376) "B\00\00\000\003\001\003\005\006\00d\005\005\002\002\005\007\00c\003\000\00f\008\001\00c\00a\006\00c\006\005\008\002\00e\000\004\000\007\008\001\008\00f\007\002\00d\00d\008\008\008\003\00d\00e\009\001\00d\008\001\002\003\00b\002\006\00d\007\00b\00b\000\00c\008\002\004\000\00")
 (data (i32.const 512) "B\00\00\000\003\00a\00a\001\00c\002\004\00a\00f\009\00d\002\00e\00c\002\00d\009\001\009\008\005\00c\004\00f\000\001\007\001\00a\00f\00b\00b\00e\007\00a\00f\003\006\00c\002\005\002\00c\008\00d\000\005\00d\004\00f\006\006\00c\008\001\00a\00c\005\00d\00d\000\009\00e\00f\007\00")
 (data (i32.const 648) "B\00\00\000\003\00f\001\006\007\00c\005\008\001\007\00c\006\005\006\00e\00e\002\00f\00b\009\00c\001\00c\00a\007\003\008\00a\00f\00f\00f\009\007\00f\00e\005\009\001\006\00e\001\008\005\000\00f\001\00d\00e\003\00e\00c\007\003\001\00d\00a\004\00e\008\00e\008\006\007\004\00c\00")
 (data (i32.const 784) "\1c\00\00\00s\00a\00m\00p\00l\00e\00s\00/\00m\00u\00l\00t\00i\00s\00i\00g\00/\00m\00u\00l\00t\00i\00s\00i\00g\00.\00t\00s\00")
 (import "env" "mem_init_buddy" (func $assembly/allocators/buddy/mem_init (param i32)))
 (import "env" "table_load_typed_arr" (func $assembly/helpers/arrays/loadTypedArray<~lib/typedarray/Uint8Array> (param i32) (result i32)))
 (import "env" "abort" (func $~lib/env/abort (param i32 i32 i32 i32)))
 (import "env" "mem_alloc" (func $assembly/allocators/base/mem_alloc (param i32) (result i32)))
 (import "env" "hex_decode_utf16" (func $assembly/helpers/strings/hexDecode (param i32) (result i32)))
 (import "env" "verify_multi_sig" (func $assembly/helpers/crypto/verifyMultiSig (param i32 i32) (result i32)))
 (table $0 1 funcref)
 (elem (i32.const 0) $null)
 (global $~lib/memory/HEAP_BASE i32 (i32.const 844))
 (export "memory" (memory $0))
 (export "table" (table $0))
 (start $start)
 (func $start:assembly/allocators/buddy (; 6 ;) (type $FUNCSIG$v)
  global.get $~lib/memory/HEAP_BASE
  call $assembly/allocators/buddy/mem_init
 )
 (func $~lib/internal/arraybuffer/computeSize (; 7 ;) (type $FUNCSIG$ii) (param $0 i32) (result i32)
  i32.const 1
  i32.const 32
  local.get $0
  i32.const 8
  i32.add
  i32.const 1
  i32.sub
  i32.clz
  i32.sub
  i32.shl
 )
 (func $assembly/allocators/base/__memory_allocate (; 8 ;) (type $FUNCSIG$ii) (param $0 i32) (result i32)
  local.get $0
  call $assembly/allocators/base/mem_alloc
 )
 (func $~lib/internal/arraybuffer/allocateUnsafe (; 9 ;) (type $FUNCSIG$ii) (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  local.get $0
  i32.const 1073741816
  i32.le_u
  i32.eqz
  if
   i32.const 0
   i32.const 40
   i32.const 26
   i32.const 2
   call $~lib/env/abort
   unreachable
  end
  block $~lib/memory/memory.allocate|inlined.0 (result i32)
   local.get $0
   call $~lib/internal/arraybuffer/computeSize
   local.set $2
   local.get $2
   call $assembly/allocators/base/__memory_allocate
   br $~lib/memory/memory.allocate|inlined.0
  end
  local.set $1
  local.get $1
  local.get $0
  i32.store
  local.get $1
 )
 (func $~lib/memory/memory.allocate (; 10 ;) (type $FUNCSIG$ii) (param $0 i32) (result i32)
  local.get $0
  call $assembly/allocators/base/__memory_allocate
  return
 )
 (func $~lib/internal/memory/memset (; 11 ;) (type $FUNCSIG$viii) (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i64)
  local.get $2
  i32.eqz
  if
   return
  end
  local.get $0
  local.get $1
  i32.store8
  local.get $0
  local.get $2
  i32.add
  i32.const 1
  i32.sub
  local.get $1
  i32.store8
  local.get $2
  i32.const 2
  i32.le_u
  if
   return
  end
  local.get $0
  i32.const 1
  i32.add
  local.get $1
  i32.store8
  local.get $0
  i32.const 2
  i32.add
  local.get $1
  i32.store8
  local.get $0
  local.get $2
  i32.add
  i32.const 2
  i32.sub
  local.get $1
  i32.store8
  local.get $0
  local.get $2
  i32.add
  i32.const 3
  i32.sub
  local.get $1
  i32.store8
  local.get $2
  i32.const 6
  i32.le_u
  if
   return
  end
  local.get $0
  i32.const 3
  i32.add
  local.get $1
  i32.store8
  local.get $0
  local.get $2
  i32.add
  i32.const 4
  i32.sub
  local.get $1
  i32.store8
  local.get $2
  i32.const 8
  i32.le_u
  if
   return
  end
  i32.const 0
  local.get $0
  i32.sub
  i32.const 3
  i32.and
  local.set $3
  local.get $0
  local.get $3
  i32.add
  local.set $0
  local.get $2
  local.get $3
  i32.sub
  local.set $2
  local.get $2
  i32.const -4
  i32.and
  local.set $2
  i32.const -1
  i32.const 255
  i32.div_u
  local.get $1
  i32.const 255
  i32.and
  i32.mul
  local.set $4
  local.get $0
  local.get $4
  i32.store
  local.get $0
  local.get $2
  i32.add
  i32.const 4
  i32.sub
  local.get $4
  i32.store
  local.get $2
  i32.const 8
  i32.le_u
  if
   return
  end
  local.get $0
  i32.const 4
  i32.add
  local.get $4
  i32.store
  local.get $0
  i32.const 8
  i32.add
  local.get $4
  i32.store
  local.get $0
  local.get $2
  i32.add
  i32.const 12
  i32.sub
  local.get $4
  i32.store
  local.get $0
  local.get $2
  i32.add
  i32.const 8
  i32.sub
  local.get $4
  i32.store
  local.get $2
  i32.const 24
  i32.le_u
  if
   return
  end
  local.get $0
  i32.const 12
  i32.add
  local.get $4
  i32.store
  local.get $0
  i32.const 16
  i32.add
  local.get $4
  i32.store
  local.get $0
  i32.const 20
  i32.add
  local.get $4
  i32.store
  local.get $0
  i32.const 24
  i32.add
  local.get $4
  i32.store
  local.get $0
  local.get $2
  i32.add
  i32.const 28
  i32.sub
  local.get $4
  i32.store
  local.get $0
  local.get $2
  i32.add
  i32.const 24
  i32.sub
  local.get $4
  i32.store
  local.get $0
  local.get $2
  i32.add
  i32.const 20
  i32.sub
  local.get $4
  i32.store
  local.get $0
  local.get $2
  i32.add
  i32.const 16
  i32.sub
  local.get $4
  i32.store
  i32.const 24
  local.get $0
  i32.const 4
  i32.and
  i32.add
  local.set $3
  local.get $0
  local.get $3
  i32.add
  local.set $0
  local.get $2
  local.get $3
  i32.sub
  local.set $2
  local.get $4
  i64.extend_i32_u
  local.get $4
  i64.extend_i32_u
  i64.const 32
  i64.shl
  i64.or
  local.set $5
  block $break|0
   loop $continue|0
    local.get $2
    i32.const 32
    i32.ge_u
    if
     block
      local.get $0
      local.get $5
      i64.store
      local.get $0
      i32.const 8
      i32.add
      local.get $5
      i64.store
      local.get $0
      i32.const 16
      i32.add
      local.get $5
      i64.store
      local.get $0
      i32.const 24
      i32.add
      local.get $5
      i64.store
      local.get $2
      i32.const 32
      i32.sub
      local.set $2
      local.get $0
      i32.const 32
      i32.add
      local.set $0
     end
     br $continue|0
    end
   end
  end
 )
 (func $~lib/array/Array<~lib/typedarray/Uint8Array>#constructor (; 12 ;) (type $FUNCSIG$iii) (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  local.get $1
  i32.const 268435454
  i32.gt_u
  if
   i32.const 0
   i32.const 8
   i32.const 45
   i32.const 39
   call $~lib/env/abort
   unreachable
  end
  local.get $1
  i32.const 2
  i32.shl
  local.set $2
  local.get $2
  call $~lib/internal/arraybuffer/allocateUnsafe
  local.set $3
  block (result i32)
   local.get $0
   i32.eqz
   if
    i32.const 8
    call $~lib/memory/memory.allocate
    local.set $0
   end
   local.get $0
   i32.const 0
   i32.store
   local.get $0
   i32.const 0
   i32.store offset=4
   local.get $0
  end
  local.get $3
  i32.store
  local.get $0
  local.get $1
  i32.store offset=4
  block $~lib/memory/memory.fill|inlined.0
   local.get $3
   i32.const 8
   i32.add
   local.set $4
   i32.const 0
   local.set $5
   local.get $2
   local.set $6
   local.get $4
   local.get $5
   local.get $6
   call $~lib/internal/memory/memset
  end
  local.get $0
 )
 (func $~lib/array/Array<~lib/typedarray/Uint8Array>#__unchecked_set (; 13 ;) (type $FUNCSIG$viii) (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  local.get $0
  i32.load
  local.set $3
  local.get $1
  local.set $4
  local.get $2
  local.set $5
  i32.const 0
  local.set $6
  local.get $3
  local.get $4
  i32.const 2
  i32.shl
  i32.add
  local.get $6
  i32.add
  local.get $5
  i32.store offset=8
 )
 (func $samples/multisig/multisig/checkMultiSig (; 14 ;) (type $FUNCSIG$v)
  (local $0 i32)
  (local $1 i32)
  (local $2 i32)
  block (result i32)
   i32.const 0
   i32.const 3
   call $~lib/array/Array<~lib/typedarray/Uint8Array>#constructor
   local.set $1
   local.get $1
   i32.const 0
   block $assembly/helpers/arrays/loadUint8Array|inlined.0 (result i32)
    i32.const 0
    local.set $0
    local.get $0
    call $assembly/helpers/arrays/loadTypedArray<~lib/typedarray/Uint8Array>
   end
   call $~lib/array/Array<~lib/typedarray/Uint8Array>#__unchecked_set
   local.get $1
   i32.const 1
   block $assembly/helpers/arrays/loadUint8Array|inlined.1 (result i32)
    i32.const 1
    local.set $0
    local.get $0
    call $assembly/helpers/arrays/loadTypedArray<~lib/typedarray/Uint8Array>
   end
   call $~lib/array/Array<~lib/typedarray/Uint8Array>#__unchecked_set
   local.get $1
   i32.const 2
   block $assembly/helpers/arrays/loadUint8Array|inlined.2 (result i32)
    i32.const 2
    local.set $0
    local.get $0
    call $assembly/helpers/arrays/loadTypedArray<~lib/typedarray/Uint8Array>
   end
   call $~lib/array/Array<~lib/typedarray/Uint8Array>#__unchecked_set
   local.get $1
  end
  local.set $1
  block (result i32)
   i32.const 0
   i32.const 5
   call $~lib/array/Array<~lib/typedarray/Uint8Array>#constructor
   local.set $2
   local.get $2
   i32.const 0
   i32.const 104
   call $assembly/helpers/strings/hexDecode
   call $~lib/array/Array<~lib/typedarray/Uint8Array>#__unchecked_set
   local.get $2
   i32.const 1
   i32.const 240
   call $assembly/helpers/strings/hexDecode
   call $~lib/array/Array<~lib/typedarray/Uint8Array>#__unchecked_set
   local.get $2
   i32.const 2
   i32.const 376
   call $assembly/helpers/strings/hexDecode
   call $~lib/array/Array<~lib/typedarray/Uint8Array>#__unchecked_set
   local.get $2
   i32.const 3
   i32.const 512
   call $assembly/helpers/strings/hexDecode
   call $~lib/array/Array<~lib/typedarray/Uint8Array>#__unchecked_set
   local.get $2
   i32.const 4
   i32.const 648
   call $assembly/helpers/strings/hexDecode
   call $~lib/array/Array<~lib/typedarray/Uint8Array>#__unchecked_set
   local.get $2
  end
  local.set $2
  local.get $1
  local.get $2
  call $assembly/helpers/crypto/verifyMultiSig
  i32.const 0
  i32.ne
  i32.eqz
  if
   i32.const 0
   i32.const 784
   i32.const 33
   i32.const 4
   call $~lib/env/abort
   unreachable
  end
 )
 (func $start:samples/multisig/multisig (; 15 ;) (type $FUNCSIG$v)
  call $start:assembly/allocators/buddy
  call $samples/multisig/multisig/checkMultiSig
 )
 (func $start (; 16 ;) (type $FUNCSIG$v)
  call $start:samples/multisig/multisig
 )
 (func $null (; 17 ;) (type $FUNCSIG$v)
 )
)
