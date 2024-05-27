use benchmark_tests::sort_arr;
use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

// bechmark function
// execute benchmarks with "cargo bench" command
fn sort_arr_benchmark(c: &mut Criterion) {
    let mut arr = black_box( // prevents compiler optimizations to arrays
        [6, 2, 4, 1, 9, -2, 5]
    );

    c.bench_function(
        "sorting algorithm", 
        |b| b.iter(|| sort_arr(&mut arr))
    );
}

criterion_group!(benches, sort_arr_benchmark); // group a list of beanchmark function namess into a group
criterion_main!(benches); // create a main function that executes all group benchmarks