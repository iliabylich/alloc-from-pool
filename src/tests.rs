use crate::*;

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[test]
fn test_alloc() {
    let pool = Pool::new();

    // perform allocation
    let p1 = pool.alloc(Point { x: 1, y: 1 });
    assert_eq!(p1, &Point { x: 1, y: 1 });
    assert_eq!(pool.total_allocations(), 1);
    assert_eq!(pool.len(), 0);

    // perform allocation
    let p2 = pool.alloc(Point { x: 2, y: 2 });
    assert_eq!(p2, &Point { x: 2, y: 2 });
    assert_eq!(pool.total_allocations(), 2);
    assert_eq!(pool.len(), 0);

    // return back to pool
    drop(p1);
    assert_eq!(pool.len(), 1);

    // allocate from pool
    let p3 = pool.alloc(Point { x: 3, y: 3 });
    assert_eq!(p3, &Point { x: 3, y: 3 });
    assert_eq!(pool.total_allocations(), 2);
    assert_eq!(pool.len(), 0);

    // items can be cloned like other nested structs
    let p4 = p3.clone();
    assert_eq!(p4, &Point { x: 3, y: 3 });
    assert_eq!(pool.total_allocations(), 3);
    drop(p4);
    assert_eq!(pool.len(), 1);

    // return back to pool
    drop(p2);
    assert_eq!(pool.len(), 2);

    // return back to pool
    drop(p3);
    assert_eq!(pool.len(), 3);

    // still only 2 allocations (2 initial, 1 recycled)
    assert_eq!(pool.total_allocations(), 3);

    // allocate using factory
    let factory = pool.factory();
    let p5 = factory.alloc(Point { x: 5, y: 5 });
    assert_eq!(p5, &Point { x: 5, y: 5 });
}
