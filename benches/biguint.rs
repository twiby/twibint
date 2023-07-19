use twibint::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn add<const N: usize>(c: &mut Criterion) {
    let mut name = "add ".to_string();
    name.push_str(&N.to_string());

    let n1 = gen_random_biguint(N);
    let n2 = gen_random_biguint(N);
    c.bench_function(name.as_str(), |b| b.iter(|| black_box(&n1 + &n2)));
}

criterion_group!(
    biguint_add,
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
    let mut name = "sub ".to_string();
    name.push_str(&N.to_string());

    let n1 = gen_random_biguint(N + 1);
    let n2 = gen_random_biguint(N);
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
    let mut name = "mul ".to_string();
    name.push_str(&N.to_string());

    let n1 = gen_random_biguint(N);
    let n2 = gen_random_biguint(N);
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

criterion_main!(biguint_add, biguint_sub, biguint_mul);
