use bencher::gen_random_biguint;
use bencher::GetNbBits;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn add<const N: usize>(c: &mut Criterion) {
    let n1 = gen_random_biguint(N);
    let n2 = gen_random_biguint(N);

    let mut name = "add ".to_string();
    name.push_str(&n1.get_nb_bits().to_string());
    name.push('+');
    name.push_str(&n2.get_nb_bits().to_string());

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
    add<100_000_000>,
);

pub fn sub<const N: usize>(c: &mut Criterion) {
    let n2 = gen_random_biguint(N);
    let n1 = &n2 + u64::MAX;

    let mut name = "sub ".to_string();
    name.push_str(&n1.get_nb_bits().to_string());
    name.push('-');
    name.push_str(&n2.get_nb_bits().to_string());

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
    sub<100_000_000>,
);

pub fn mul<const N: usize>(c: &mut Criterion) {
    let n1 = gen_random_biguint(N);
    let n2 = gen_random_biguint(N);

    let mut name = "mul ".to_string();
    name.push_str(&n2.get_nb_bits().to_string());
    name.push('x');
    name.push_str(&n1.get_nb_bits().to_string());

    c.bench_function(name.as_str(), |b| b.iter(|| black_box(&n1 * &n2)));
}

criterion_group!(
    biguint_mul,
    mul<30>,
    mul<100>,
    mul<300>,
    mul<1_000>,
    mul<3_000>,
    mul<10_000>,
    mul<30_000>,
);

pub fn asymetric_mul<const N: usize, const N2: usize>(c: &mut Criterion) {
    let n1 = gen_random_biguint(N);
    let n2 = gen_random_biguint(N2);

    let mut name = "asymetric_mul ".to_string();
    name.push_str(&n2.get_nb_bits().to_string());
    name.push('x');
    name.push_str(&n1.get_nb_bits().to_string());

    c.bench_function(name.as_str(), |b| b.iter(|| black_box(&n1 * &n2)));
}

criterion_group!(
    biguint_asymetric_mul,
    asymetric_mul<30, 3>,
    asymetric_mul<100, 9>,
    asymetric_mul<300, 27>,
    asymetric_mul<1_000, 92>,
    asymetric_mul<3_000, 287>,
    asymetric_mul<10_000, 1001>,
    asymetric_mul<30_000, 3027>,
);

pub fn div<const N: usize, const N2: usize>(c: &mut Criterion) {
    let n1 = gen_random_biguint(N);
    let n2 = gen_random_biguint(N2);

    let mut name = "div ".to_string();
    name.push_str(&n1.get_nb_bits().to_string());
    name.push('/');
    name.push_str(&n2.get_nb_bits().to_string());

    c.bench_function(name.as_str(), |b| b.iter(|| black_box(&n1 / &n2)));
}

criterion_group!(
    biguint_div,
    div<30, 3>,
    div<100, 9>,
    div<300, 27>,
    div<1_000, 92>,
    div<3_000, 287>,
    div<10_000, 1001>,
    div<30_000, 3027>,
    div<100_000, 10_011>,
    div<300_000, 30_123>,
);

criterion_main!(
    biguint_add,
    biguint_sub,
    biguint_mul,
    biguint_asymetric_mul,
    biguint_div
);
