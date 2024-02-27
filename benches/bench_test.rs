use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn fibonacci_slow(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci_slow(n-1) + fibonacci_slow(n-2),
    }
}

fn fibonacci_fast(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");

    for i in [20u64, 21u64].iter() {
        group.bench_with_input(BenchmarkId::new("Recursive", i), i, 
            |b, i| b.iter(|| fibonacci_slow(*i)));
        group.bench_with_input(BenchmarkId::new("Iterative", i), i, 
            |b, i| b.iter(|| fibonacci_fast(*i)));
    }
    group.finish();
}


criterion_group!{
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(500);
    targets = bench_fibs
}
criterion_main!(benches);