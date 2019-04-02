(module
 (type $FUNCSIG$vi (func (param i32)))
 (type $FUNCSIG$v (func))
 (type $FUNCSIG$ii (func (param i32) (result i32)))
 (type $FUNCSIG$iii (func (param i32 i32) (result i32)))
 (import "env" "memory" (memory $0 1))
 (data (i32.const 8) "(\00\00\002\00e\00f\001\00e\00a\00c\00c\008\00c\00a\00d\002\009\00a\002\007\00a\005\004\003\001\002\007\003\001\00d\006\00f\003\006\002\004\00e\000\001\003\00e\004\006\00")
 (import "env" "mem_init_arena" (func $assembly/allocators/arena/mem_init (param i32)))
 (import "env" "mem_alloc" (func $assembly/allocators/arena/mem_alloc (param i32) (result i32)))
 (import "env" "table_load_typed_arr" (func $samples/as-p2pkh/p2pkh/loadArray (param i32 i32) (result i32)))
 (import "env" "hash160" (func $samples/as-p2pkh/p2pkh/hash (param i32 i32) (result i32)))
 (import "env" "hex_decode_utf16" (func $samples/as-p2pkh/p2pkh/hexDecode (param i32 i32) (result i32)))
 (import "env" "compare" (func $samples/as-p2pkh/p2pkh/compare (param i32 i32) (result i32)))
 (import "env" "verify_sig" (func $samples/as-p2pkh/p2pkh/verifySig (param i32 i32) (result i32)))
 (table $0 1 funcref)
 (elem (i32.const 0) $null)
 (global $samples/as-p2pkh/p2pkh/localHash i32 (i32.const 8))
 (global $~lib/memory/HEAP_BASE i32 (i32.const 92))
 (export "memory" (memory $0))
 (export "table" (table $0))
 (start $start)
 (func $start:assembly/allocators/arena (; 7 ;) (type $FUNCSIG$v)
  global.get $~lib/memory/HEAP_BASE
  call $assembly/allocators/arena/mem_init
 )
 (func $assembly/allocators/arena/__memory_allocate (; 8 ;) (type $FUNCSIG$ii) (param $0 i32) (result i32)
  local.get $0
  call $assembly/allocators/arena/mem_alloc
 )
 (func $samples/as-p2pkh/p2pkh/start (; 9 ;) (type $FUNCSIG$v)
  (local $0 i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  block $~lib/memory/memory.allocate|inlined.0 (result i32)
   i32.const 256
   local.set $0
   local.get $0
   call $assembly/allocators/arena/__memory_allocate
   br $~lib/memory/memory.allocate|inlined.0
  end
  local.set $0
  block $~lib/memory/memory.allocate|inlined.1 (result i32)
   i32.const 256
   local.set $1
   local.get $1
   call $assembly/allocators/arena/__memory_allocate
   br $~lib/memory/memory.allocate|inlined.1
  end
  local.set $1
  block $~lib/memory/memory.allocate|inlined.2 (result i32)
   i32.const 256
   local.set $2
   local.get $2
   call $assembly/allocators/arena/__memory_allocate
   br $~lib/memory/memory.allocate|inlined.2
  end
  local.set $2
  block $~lib/memory/memory.allocate|inlined.3 (result i32)
   i32.const 256
   local.set $3
   local.get $3
   call $assembly/allocators/arena/__memory_allocate
   br $~lib/memory/memory.allocate|inlined.3
  end
  local.set $3
  i32.const 0
  local.get $0
  call $samples/as-p2pkh/p2pkh/loadArray
  drop
  i32.const 1
  local.get $1
  call $samples/as-p2pkh/p2pkh/loadArray
  drop
  local.get $1
  local.get $2
  call $samples/as-p2pkh/p2pkh/hash
  drop
  global.get $samples/as-p2pkh/p2pkh/localHash
  local.get $3
  call $samples/as-p2pkh/p2pkh/hexDecode
  drop
  local.get $3
  local.get $2
  call $samples/as-p2pkh/p2pkh/compare
  i32.const 0
  i32.ne
  i32.eqz
  if
   unreachable
  end
  local.get $0
  local.get $1
  call $samples/as-p2pkh/p2pkh/verifySig
  i32.const 0
  i32.ne
  i32.eqz
  if
   unreachable
  end
 )
 (func $start:samples/as-p2pkh/p2pkh (; 10 ;) (type $FUNCSIG$v)
  call $start:assembly/allocators/arena
  call $samples/as-p2pkh/p2pkh/start
 )
 (func $start (; 11 ;) (type $FUNCSIG$v)
  call $start:samples/as-p2pkh/p2pkh
 )
 (func $null (; 12 ;) (type $FUNCSIG$v)
 )
)
