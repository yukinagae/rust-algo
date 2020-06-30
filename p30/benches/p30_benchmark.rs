extern crate p30;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use p30::fact::fact;
use p30::fib::{fib, fib_memo};

pub fn criterion_benchmark(c: &mut Criterion) {
    // fact 10                 time:   [9.8255 ns 10.315 ns 10.861 ns]
    // Found 7 outliers among 100 measurements (7.00%)
    //   5 (5.00%) high mild
    //   2 (2.00%) high severe
    //
    // fact 100                time:   [24.385 ns 24.669 ns 25.030 ns]
    // Found 14 outliers among 100 measurements (14.00%)
    //   5 (5.00%) high mild
    //   9 (9.00%) high severe
    //
    // fact 1000               time:   [246.04 ns 246.85 ns 247.74 ns]
    // Found 10 outliers among 100 measurements (10.00%)
    //   6 (6.00%) high mild
    //   4 (4.00%) high severe
    //
    // fact 10000              time:   [2.4369 us 2.4411 us 2.4461 us]
    // Found 16 outliers among 100 measurements (16.00%)
    //   7 (7.00%) high mild
    //   9 (9.00%) high severe
    // c.bench_function("fact 10", |b| b.iter(|| fact(black_box(10))));
    // c.bench_function("fact 100", |b| b.iter(|| fact(black_box(100))));
    // c.bench_function("fact 1000", |b| b.iter(|| fact(black_box(1000))));
    // c.bench_function("fact 10000", |b| b.iter(|| fact(black_box(10000))));

    // fib 10                  time:   [310.03 ns 319.18 ns 329.72 ns]
    //                         change: [-0.7187% +2.3312% +5.3801%] (p = 0.14 > 0.05)
    //                         No change in performance detected.
    // Found 12 outliers among 100 measurements (12.00%)
    //   4 (4.00%) high mild
    //   8 (8.00%) high severe
    // fib 20                  time:   [37.608 us 37.981 us 38.429 us]
    //                         change: [-21.940% -12.128% -2.2342%] (p = 0.03 < 0.05)
    //                         Performance has improved.
    // Found 8 outliers among 100 measurements (8.00%)
    //   4 (4.00%) high mild
    //   4 (4.00%) high severe
    c.bench_function("fib 10", |b| b.iter(|| fib(black_box(10))));
    c.bench_function("fib 20", |b| b.iter(|| fib(black_box(20))));

    // fib_memo 10             time:   [197.12 ns 234.09 ns 280.06 ns]
    // Found 17 outliers among 100 measurements (17.00%)
    //   2 (2.00%) high mild
    //   15 (15.00%) high severe
    // fib_memo 20             time:   [223.19 ns 226.89 ns 231.89 ns]
    // Found 9 outliers among 100 measurements (9.00%)
    //   3 (3.00%) high mild
    //   6 (6.00%) high severe
    c.bench_function("fib_memo 10", |b| {
        b.iter(|| fib_memo(black_box(10), &mut vec![0; 10 + 1]))
    });
    c.bench_function("fib_memo 20", |b| {
        b.iter(|| fib_memo(black_box(20), &mut vec![0; 20 + 1]))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
