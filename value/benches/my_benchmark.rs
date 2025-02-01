use criterion::{BatchSize, Criterion, black_box, criterion_group, criterion_main};
use rand::prelude::*;
use std::str::FromStr;
use value::Number;

fn num_to_number(input: f64) -> Number {
    let string = input.to_string();
    Number::from_str(&string).expect("should be withing value max length")
}

pub fn criterion_benchmark(c: &mut Criterion) {
    bench_mark_add(c, -30.0..30.);
    bench_mark_add(c, -1000.0..1000.);
    bench_mark_add(c, -1000000.0..1000000.);

    bench_mark_eq_trivial(c, -100.0..100.0);
    bench_mark_eq_trivial(c, -100000000.0..100000000.0);

    bench_mark_eq(c, -100.0..100.0);
    bench_mark_eq(c, -100000000.0..100000000.0);
}

fn bench_mark_add(c: &mut Criterion, range: std::ops::Range<f64>) {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(10);
    c.bench_function(&format!("add in range :{range:?}"), |b| {
        b.iter_batched(
            || {
                (
                    num_to_number(rng.random_range(range.clone())),
                    num_to_number(rng.random_range(range.clone())),
                )
            },
            |(a, b)| black_box(a) + black_box(b),
            BatchSize::SmallInput,
        )
    });
}

fn bench_mark_eq_trivial(c: &mut Criterion, range: std::ops::Range<f64>) {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(10);
    c.bench_function(&format!("eq trivial in range :{range:?}"), |b| {
        b.iter_batched(
            || {
                let x = num_to_number(rng.random_range(range.clone()));
                (x.clone(), x)
            },
            |(a, b)| black_box(a) == black_box(b),
            BatchSize::SmallInput,
        )
    });
}

fn bench_mark_eq(c: &mut Criterion, range: std::ops::Range<f64>) {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(10);
    let long_number =
        Number::from_str("1000000000000000000000000000.00000000000000000001").unwrap();
    c.bench_function(&format!("eq in range :{range:?}"), |b| {
        b.iter_batched(
            || {
                let x = num_to_number(rng.random_range(range.clone()));
                //Internally numbers can have an arbitrary amount of leading/trailing "zeros"
                //(if negative the leading/trailing digit is 9)
                //By adding adding and subtracting a long number we are forcefully increasing the
                //number of leading and trailing digits.
                (x.clone() + long_number.clone() - long_number.clone(), x)
            },
            |(a, b)| black_box(a) == black_box(b),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
