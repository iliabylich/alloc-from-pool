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

// it also possible to create a Factory based on a pool
// that holds a reference to initial Pool
let factory = pool.factory();
// Factory has the same `alloc` method:
let foo3 = factory.alloc(Foo([3; 50]));
// it uses the same pool, and also returns back to the same pool
drop(foo3);
```

### Benchmarks

You can run `cargo bench`:

```text
test alloc_with_box  ... bench:          56 ns/iter (+/- 6)
test alloc_with_pool ... bench:           7 ns/iter (+/- 0)
```
