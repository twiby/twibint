use twibint;
/// Choosing a digit representation
type BigUint = twibint::BigUint<u64>;

const NB_BITS_TO_COMPUTE: usize = 10000;

/// This computes the bits of sqrt_2 in a biguint
///
/// This does so bit by bit, shifting and comparing to a odd power of 2
/// Surely not the optimal algorithm
fn main() {
    let mut two = BigUint::new(2);
    let mut sqrt_2_approx = BigUint::new(1);
    let mut two_approx =
        BigUint::new(1).with_capacity((NB_BITS_TO_COMPUTE + 1) * (NB_BITS_TO_COMPUTE + 1));

    for _ in 0..NB_BITS_TO_COMPUTE / 2 {
        two <<= 4;
        sqrt_2_approx <<= 2;
        two_approx <<= 4;

        while two_approx < two {
            two_approx += &sqrt_2_approx;
            sqrt_2_approx += 1;
            two_approx += &sqrt_2_approx;
        }

        if two_approx >= two {
            two_approx -= &sqrt_2_approx;
            sqrt_2_approx -= 1;
            two_approx -= &sqrt_2_approx;
        }
    }

    // We check that we haven't done an oopsie daisy
    // sqrt_2_appprox should be the closest integer below the square
    // root of an odd power of 2
    let ret = &sqrt_2_approx * &sqrt_2_approx;
    let ret_p1 = (&sqrt_2_approx + 1) * (&sqrt_2_approx + 1);
    let next_pow2 = BigUint::new(1) << ret.nb_bits();
    assert!(sqrt_2_approx.nb_bits() % 2 == 1);
    assert!(ret.nb_bits() % 2 == 1);
    assert!(ret < next_pow2);
    assert!(ret_p1 > next_pow2);

    println!("nb bits computed {:?}", sqrt_2_approx.nb_bits() - 1);
    sqrt_2_approx
        .write_to_file("examples_sqrt_2.tw")
        .expect("Exporting failed");
}
