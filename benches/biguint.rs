use bigint::*;

use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::{thread_rng, Rng};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn gen_n_random_values<T>(n: usize) -> Vec<T>
where
    Standard: Distribution<T>,
{
    let mut ret = Vec::<T>::with_capacity(n);
    for _ in 0..n {
        ret.push(thread_rng().gen::<T>());
    }
    ret
}

pub fn add<const N: usize>(c: &mut Criterion) {
    let mut name = "add ".to_string();
    name.push_str(&N.to_string());

    let n1 = biguint!(gen_n_random_values::<u32>(N));
    let n2 = biguint!(gen_n_random_values::<u32>(N));
    c.bench_function(name.as_str(), |b| b.iter(|| black_box(&n1 + &n2)));
}

criterion_group!(
    biguint_add,
    add<1_000>,
    add<5_000>,
    add<10_000>,
    add<50_000>,
    add<100_000>,
    add<500_000>,
    add<1_000_000>,
    add<5_000_000>,
    add<10_000_000>,
    add<50_000_000>,
    add<100_000_000>
);

pub fn mul<const N: usize>(c: &mut Criterion) {
    let mut name = "mul ".to_string();
    name.push_str(&N.to_string());

    let n1 = biguint!(gen_n_random_values::<u32>(N));
    let n2 = biguint!(gen_n_random_values::<u32>(N));
    c.bench_function(name.as_str(), |b| b.iter(|| black_box(&n1 * &n2)));
}

criterion_group!(
    biguint_mul,
    mul<10>,
    mul<50>,
    mul<100>,
    mul<500>,
    mul<1_000>,
    mul<5_000>,
    mul<10_000>
);

criterion_main!(biguint_add, biguint_mul);
