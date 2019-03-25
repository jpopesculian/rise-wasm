(module
  (func $load
    (import "imports" "stack_to_mem") (param i32))
  (import "memory" "default" (memory 1))

  (func $dec (param $num i32) (result i32)
    (i32.sub (get_local $num) (i32.const 1)))

  (func (export "main") (result i32) (local $loop-left i32)
    ;; stack: [loop-left]
    (call $load (i32.const 0)) ;; get loop-left param
    ;; stack: []
    (set_local $loop-left (i32.load (i32.const 0)))
    (block
    (loop
      (br_if 1 (i32.le_s (get_local $loop-left) (i32.const 0)))
      (set_local $loop-left (call $dec (get_local $loop-left)))
      (br 0))
    )
    (i32.const 1)))
