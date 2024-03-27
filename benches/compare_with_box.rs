use alloc_from_pool::Pool;
use bencher::{benchmark_group, benchmark_main, Bencher};
use std::hint::black_box;

struct Dummy(#[allow(dead_code)] [u8; 50]);

impl Dummy {
    fn new(i: u64) -> Self {
        Self([(i % 255) as u8; 50])
    }
}

fn alloc_with_pool(bench: &mut Bencher) {
    let pool = Pool::new();
    bench.iter(|| {
        let pooled = pool.alloc(Dummy::new(42));
        black_box(pooled);
    });
}

fn alloc_with_box(bench: &mut Bencher) {
    bench.iter(|| {
        let boxed = Box::new(Dummy::new(42));
        black_box(boxed);
    });
}

benchmark_group!(benches, alloc_with_pool, alloc_with_box);
benchmark_main!(benches);
