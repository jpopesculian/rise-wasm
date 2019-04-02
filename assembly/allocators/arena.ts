@external("env", "mem_init_arena")
declare function mem_init(index: usize): void;

@external("env", "mem_alloc")
declare function mem_alloc(index: usize): u32;

@external("env", "mem_free")
declare function mem_free(index: usize): void;

@external("env", "mem_reset")
declare function mem_reset(): void;


@global export function __memory_allocate(size: usize): usize {
    return mem_alloc(size);
}

@global export function __memory_free(ptr: usize): void {
    return mem_free(ptr);
}

@global export function __memory_reset(): void {
    return mem_reset();
}

mem_init(HEAP_BASE);
