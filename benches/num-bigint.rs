use num_bigint::BigUint;

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
    let mut name = "num-bigint add ".to_string();
    name.push_str(&N.to_string());

    let n1 = BigUint::new(gen_n_random_values::<u32>(N));
    let n2 = BigUint::new(gen_n_random_values::<u32>(N));
    c.bench_function(name.as_str(), |b| b.iter(|| black_box(&n1 + &n2)));
}

criterion_group!(
    num_bigint_add,
    add<1_000>,
    add<3_000>,
    add<10_000>,
    add<30_000>,
    add<100_000>,
    add<300_000>,
    add<1_000_000>,
    add<3_000_000>,
    add<10_000_000>,
    add<30_000_000>,
    add<100_000_000>
);

pub fn sub<const N: usize>(c: &mut Criterion) {
    let mut name = "num-bigint sub ".to_string();
    name.push_str(&N.to_string());

    let n1 = BigUint::new(gen_n_random_values::<u32>(N+1));
    let n2 = BigUint::new(gen_n_random_values::<u32>(N));
    c.bench_function(name.as_str(), |b| b.iter(|| black_box(&n1 - &n2)));
}

criterion_group!(
    biguint_sub,
    sub<1_000>,
    sub<3_000>,
    sub<10_000>,
    sub<30_000>,
    sub<100_000>,
    sub<300_000>,
    sub<1_000_000>,
    sub<3_000_000>,
    sub<10_000_000>,
    sub<30_000_000>,
    sub<100_000_000>
);

pub fn mul<const N: usize>(c: &mut Criterion) {
    let mut name = "num-bigint mul ".to_string();
    name.push_str(&N.to_string());

    let n1 = BigUint::new(gen_n_random_values::<u32>(N));
    let n2 = BigUint::new(gen_n_random_values::<u32>(N));
    c.bench_function(name.as_str(), |b| b.iter(|| black_box(&n1 * &n2)));
}

criterion_group!(
    biguint_mul,
    mul<10>,
    mul<30>,
    mul<100>,
    mul<300>,
    mul<1_000>,
    mul<3_000>,
    mul<10_000>,
    mul<30_000>,
);

criterion_main!(num_bigint_add, biguint_sub, biguint_mul);
