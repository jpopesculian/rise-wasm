(module
  (func $mem_init
    (import "env" "mem_init_arena") (param i32))
  (func $load
    (import "env" "table_load_mem") (param i32) (result i32))
  (import "env" "memory" (memory 1))

  (func $dec (param $num i32) (result i32)
    (i32.sub (get_local $num) (i32.const 1)))

  (func $main
    (local $loop-left i32)
    (call $mem_init (i32.const 0))
    (set_local $loop-left (i32.load (call $load (i32.const 0))))
    (block
    (loop
      (br_if 1 (i32.le_s (get_local $loop-left) (i32.const 0)))
      (set_local $loop-left (call $dec (get_local $loop-left)))
      (br 0))
    ))
  (start $main)
)
