use twibint::traits::Digit;
use twibint::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn add<const N: usize, T: Digit>(c: &mut Criterion)
where
    rand::distributions::Standard: rand::prelude::Distribution<T>,
{
    let mut name = "add ".to_string();
    name.push_str(&N.to_string());
    name.push(' ');
    name.push_str(&T::NB_BITS.to_string());

    let n1 = gen_random_biguint::<T>(N);
    let n2 = gen_random_biguint::<T>(N);
    c.bench_function(name.as_str(), |b| b.iter(|| black_box(&n1 + &n2)));
}

criterion_group!(
    biguint_add,
    add<1_000, u32>,
    add<1_000, u64>,
    add<3_000, u32>,
    add<3_000, u64>,
    add<10_000, u32>,
    add<10_000, u64>,
    add<30_000, u32>,
    add<30_000, u64>,
    add<100_000, u32>,
    add<100_000, u64>,
    add<300_000, u32>,
    add<300_000, u64>,
    add<1_000_000, u32>,
    add<1_000_000, u64>,
    add<3_000_000, u32>,
    add<3_000_000, u64>,
    add<10_000_000, u32>,
    add<10_000_000, u64>,
    add<30_000_000, u32>,
    add<30_000_000, u64>,
    add<100_000_000, u32>,
    add<100_000_000, u64>,
);

pub fn sub<const N: usize, T: Digit>(c: &mut Criterion)
where
    rand::distributions::Standard: rand::prelude::Distribution<T>,
{
    let mut name = "sub ".to_string();
    name.push_str(&N.to_string());
    name.push(' ');
    name.push_str(&T::NB_BITS.to_string());

    let n1 = gen_random_biguint::<T>(N + 1);
    let n2 = gen_random_biguint::<T>(N);
    c.bench_function(name.as_str(), |b| b.iter(|| black_box(&n1 - &n2)));
}

criterion_group!(
    biguint_sub,
    sub<1_000, u32>,
    sub<1_000, u64>,
    sub<3_000, u32>,
    sub<3_000, u64>,
    sub<10_000, u32>,
    sub<10_000, u64>,
    sub<30_000, u32>,
    sub<30_000, u64>,
    sub<100_000, u32>,
    sub<100_000, u64>,
    sub<300_000, u32>,
    sub<300_000, u64>,
    sub<1_000_000, u32>,
    sub<1_000_000, u64>,
    sub<3_000_000, u32>,
    sub<3_000_000, u64>,
    sub<10_000_000, u32>,
    sub<10_000_000, u64>,
    sub<30_000_000, u32>,
    sub<30_000_000, u64>,
    sub<100_000_000, u32>,
    sub<100_000_000, u64>,
);

pub fn mul<const N: usize, T: Digit>(c: &mut Criterion)
where
    rand::distributions::Standard: rand::prelude::Distribution<T>,
{
    let mut name = "mul ".to_string();
    name.push_str(&N.to_string());
    name.push(' ');
    name.push_str(&T::NB_BITS.to_string());

    let n1 = gen_random_biguint::<T>(N);
    let n2 = gen_random_biguint::<T>(N);
    c.bench_function(name.as_str(), |b| b.iter(|| black_box(&n1 * &n2)));
}

criterion_group!(
    biguint_mul,
    mul<10, u32>,
    mul<10, u64>,
    mul<30, u32>,
    mul<30, u64>,
    mul<100, u32>,
    mul<100, u64>,
    mul<300, u32>,
    mul<300, u64>,
    mul<1_000, u32>,
    mul<1_000, u64>,
    mul<3_000, u32>,
    mul<3_000, u64>,
    mul<10_000, u32>,
    mul<10_000, u64>,
    mul<30_000, u32>,
    mul<30_000, u64>,
);

criterion_main!(biguint_add, biguint_sub, biguint_mul);
