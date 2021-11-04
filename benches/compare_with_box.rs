use alloc_from_pool::Pool;

#[macro_use]
extern crate bencher;

use bencher::Bencher;

#[allow(dead_code)]
struct Dummy {
    bytes: [u8; 50],
}

impl Dummy {
    fn new(i: u64) -> Self {
        Self {
            bytes: [(i % 255) as u8; 50],
        }
    }
}

// Stolen from criterion :)
fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}

#[inline(never)]
fn alloc_with_pool(bench: &mut Bencher) {
    let pool = Pool::new();
    bench.iter(|| {
        let pooled = pool.alloc(Dummy::new(42));
        black_box(pooled);
    });
}

#[inline(never)]
fn alloc_with_box(bench: &mut Bencher) {
    bench.iter(|| {
        let boxed = Box::new(Dummy::new(42));
        black_box(boxed);
    });
}

benchmark_group!(benches, alloc_with_pool, alloc_with_box);
benchmark_main!(benches);
