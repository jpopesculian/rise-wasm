import "./base";

@external("env", "mem_init_buddy")
declare function mem_init(index: usize): void;

mem_init(HEAP_BASE);
