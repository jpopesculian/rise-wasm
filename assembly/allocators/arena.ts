import "./base";

@external("env", "mem_init_arena")
declare function mem_init(index: usize): void;

mem_init(HEAP_BASE);
