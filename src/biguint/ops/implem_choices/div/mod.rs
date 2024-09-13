use crate::BigFloat;
use crate::BigInt;
use crate::BigUint;

fn div(n: &BigUint<u32>, d: &BigUint<u32>) -> BigUint<u32> {
    let t2 = -BigFloat::from(241u32) >> 7;
    let t1 = BigFloat::from(361u32) >> 7;
    let one = BigFloat::from(vec![1u32]);
    let two = BigFloat::from(vec![2u32]);

    let precision_bits = n.nb_bits() - d.nb_bits();
    let precision_digits = (precision_bits - 1) / 32 + 1;

    // All needed allocations
    let shift = d.nb_bits();
    let d = BigFloat::from(d.clone()) >> shift;
    let mut x = BigFloat::default().with_capacity((precision_digits * 2 + 2) * 32);
    let mut temp_1 = BigFloat::default().with_capacity((precision_digits * 3 + 2) * 32);
    let mut temp_2 = BigFloat::default().with_capacity((precision_digits * 2 + 2) * 32);

    // Initial "guess"
    x.set_to_mul(&d, &t2);
    x += t1;

    // First step is done like this to access initial error
    let cutoff_x = x.int.uint.val.len() - (precision_digits + 1);
    temp_1._set_to_mul(
        false,
        d.scale,
        &d.int.uint.val,
        x.int.sign,
        x.scale + (cutoff_x as isize),
        &x.int.uint.val[cutoff_x..],
    );
    temp_1 += &one;
    let cutoff_1 = temp_1.int.uint.val.len() - (precision_digits + 1);
    temp_2._set_to_mul(
        x.int.sign,
        x.scale + (cutoff_x as isize),
        &x.int.uint.val[cutoff_x..],
        temp_1.int.sign,
        temp_1.scale + (cutoff_1 as isize),
        &temp_1.int.uint.val[cutoff_1..],
    );
    let cutoff_2 = temp_2.int.uint.val.len() - (precision_digits + 1);
    x.add_assign(
        temp_2.int.sign,
        temp_2.scale + (cutoff_2 as isize),
        &temp_2.int.uint.val[cutoff_2..],
    );

    // compute number of steps
    let mut init_depth = -(temp_1.scale * 32 + (temp_1.int.uint.nb_bits() as isize)) as usize;
    let mut nb_steps = 0;
    while init_depth < precision_bits {
        nb_steps += 1;
        init_depth <<= 1;
    }
    nb_steps -= 1; // First step is already done

    // Actual newton-raphson remaining steps
    for _ in 0..nb_steps {
        let cutoff_x = x.int.uint.val.len() - (precision_digits + 1);
        temp_1._set_to_mul(
            false,
            d.scale,
            &d.int.uint.val,
            x.int.sign,
            x.scale + (cutoff_x as isize),
            &x.int.uint.val[cutoff_x..],
        );
        temp_1 += &two;
        let cutoff_1 = temp_1.int.uint.val.len() - (precision_digits + 1);
        temp_2._set_to_mul(
            x.int.sign,
            x.scale + (cutoff_x as isize),
            &x.int.uint.val[cutoff_x..],
            temp_1.int.sign,
            temp_1.scale + (cutoff_1 as isize),
            &temp_1.int.uint.val[cutoff_1..],
        );
        x.copy_from(&temp_2);
    }

    // Finish off by computing the actual division
    x >>= shift;
    let cutoff = x.int.uint.val.len() - (precision_digits + 1);
    temp_1._set_to_mul(
        true,
        x.scale + (cutoff as isize),
        &x.int.uint.val[cutoff..],
        true,
        0,
        &n.val,
    );
    BigInt::from(temp_1).uint
}
