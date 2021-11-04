## alloc-from-pool

This crate implements object pool.

### Basic usage

```rust
use alloc_from_pool::Pool;

// dummy struct
struct Foo([u8; 50]);

// pool has size = 0 by default
let pool = Pool::<Foo>::new();

// performs allocation
let foo1 = pool.alloc(Foo([1; 50]));
// returns memory back to pool
drop(foo1);

// re-uses the value that we've just returned back to pool
let foo2 = pool.alloc(Foo([2; 50]));
// returns memory back to pool
drop(foo2);
```
