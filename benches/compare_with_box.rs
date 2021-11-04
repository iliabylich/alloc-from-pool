use alloc_from_pool::Pool;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

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

#[inline(never)]
fn alloc_with_pool(pool: &Pool<Dummy>, i: u64) {
    let pooled = pool.alloc(Dummy::new(i));
    black_box(pooled);
}

#[inline(never)]
fn alloc_every_time(_pool: &Pool<Dummy>, i: u64) {
    let boxed = Box::new(Dummy::new(i));
    black_box(boxed);
}

fn bench_impls(c: &mut Criterion) {
    let mut group = c.benchmark_group("Alloc");
    let pool = Pool::new();
    for i in [20u64, 21u64].iter() {
        group.bench_with_input(BenchmarkId::new("With Pool", i), i, |b, i| {
            b.iter(|| alloc_with_pool(&pool, *i))
        });
        group.bench_with_input(BenchmarkId::new("Every Time", i), i, |b, i| {
            b.iter(|| alloc_every_time(&pool, *i))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_impls);
criterion_main!(benches);
