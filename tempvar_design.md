# Temporary Variable System Design (Demo Phase)

## Overview
This document outlines the design for implementing a block-scoped temporary variable system in Scrust **without modifying the compiler**. The implementation is a "proof of concept" demo residing in the `test/` directory.

## Constraints & Requirements
1.  **No Compiler Changes**: Use existing Scrust features.
2.  **Public Storage Only**: Uses Scratch's native public lists and variables.
3.  **Concurrency Support**: Supports multiple threads (scripts) running asynchronously, starting/stopping at different times, without data corruption.
4.  **Fixed Resources**: The number and names of backing lists/variables are deterministic and constant.

## Memory Management: Fixed-Block Allocator

To satisfy the requirement of **releasing** variables upon scope exit while maintaining **concurrency safety**, we use a **Fixed-Block Allocator**.

*   **Page Size**: Fixed at **16 items**.
*   **Lists**:
    *   `_RAM`: The main memory storage (public list).
    *   `_FREE_PAGES`: A stack (public list) of pointers to available pages that have been released.
*   **Variables**:
    *   `_HIGH_WATER`: Points to the next unallocated index in `_RAM` (public var).
    *   `_RET_VAL`: Used to simulate return values from procedures (public var).

### Allocation Logic (`sys_alloc`)
1.  Check if `_FREE_PAGES` is empty.
2.  **If not empty**: Pop the last item from `_FREE_PAGES`. This is our `base_ptr`.
3.  **If empty**: Use `_HIGH_WATER` as `base_ptr`, then increment `_HIGH_WATER` by `PAGE_SIZE`. (Grow `_RAM` with 0s).
4.  Return `base_ptr` via `_RET_VAL`.

### Deallocation Logic (`sys_free`)
1.  Push the `base_ptr` onto `_FREE_PAGES`.

This approach ensures:
*   **O(1) Allocation/Deallocation**: Fast and deterministic.
*   **Reuse**: Memory is recycled, preventing unbounded growth.
*   **Concurrency**: Atomic operations (in `warp` blocks) ensure threads don't race for the same page.

## Implementation Details

### API

```rust
// Globals
public list _RAM = [];
public list _FREE_PAGES = [];
public var _HIGH_WATER = 1;
public var _RET_VAL = 0;

#[warp]
proc sys_alloc() {
    if length_of_list(_FREE_PAGES) > 0 {
        _RET_VAL = item_of_list(_FREE_PAGES, length_of_list(_FREE_PAGES));
        delete_of_list(_FREE_PAGES, length_of_list(_FREE_PAGES));
    } else {
        _RET_VAL = _HIGH_WATER;
        _HIGH_WATER += 16;
        repeat(16) {
            add_to_list(_RAM, 0);
        }
    }
}

#[warp]
proc sys_free(ptr: number) {
    add_to_list(_FREE_PAGES, ptr);
}

proc stack_set(ptr: number, offset: number, val: number) {
    replace_item_of_list(_RAM, ptr + offset, val);
}
```

### Concurrency Model

*   **Threads**: Implemented as separate `proc` or `fn` blocks triggered by broadcast or flag click.
*   **Async Start**: Use `broadcast("thread_name")` to start a thread asynchronously.
*   **Sync Execution**: Within a thread, operations are synchronous.
*   **Memory Isolation**: Each thread allocates its own page(s) via `sys_alloc()`. The returned pointer is local to that thread's scope (passed as an argument to sub-procedures), ensuring threads do not overwrite each other's stack.

### Demo (`test/src/sprite.sr`)

The demo implements two concurrent threads:
1.  `thread_a`: Started by `main` (flag clicked).
2.  `thread_b`: Started by broadcast from `main`.

Both threads allocate memory, perform operations with different wait times (simulating async work), and free memory upon completion.
