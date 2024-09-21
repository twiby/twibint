use crate::traits::Digit;
use crate::BigFloat;
use crate::BigInt;
use crate::BigUint;

struct NewtonRaphsonMachine<'a, T: Digit> {
    x: BigFloat<T>,
    temp_1: BigFloat<T>,
    temp_2: BigFloat<T>,
    shift: usize,
    nb_steps: usize,
    precision_bits: usize,
    precision_digits: usize,

    d: BigFloat<T>,
    n: &'a BigUint<T>,
}

impl<'a, T: Digit> NewtonRaphsonMachine<'a, T> {
    fn new(n: &'a BigUint<T>, d: &'a BigUint<T>) -> Self {
        let t2 = -BigFloat::from(BigUint::<T>::from(241u32)) >> 7;
        let t1 = BigFloat::from(BigUint::<T>::from(361u32)) >> 7;

        let precision_bits = n.nb_bits() - d.nb_bits();
        let precision_digits = (precision_bits - 1) / 32 + 3;

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
        let cutoff_x = x.int.uint.val.len() - precision_digits;
        temp_1._set_to_mul(
            false,
            d.scale,
            &d.int.uint.val,
            x.int.sign,
            x.scale + (cutoff_x as isize),
            &x.int.uint.val[cutoff_x..],
        );
        temp_1 += T::ONE;
        let cutoff_1 = temp_1.int.uint.val.len() - precision_digits;
        temp_2._set_to_mul(
            x.int.sign,
            x.scale + (cutoff_x as isize),
            &x.int.uint.val[cutoff_x..],
            temp_1.int.sign,
            temp_1.scale + (cutoff_1 as isize),
            &temp_1.int.uint.val[cutoff_1..],
        );
        let cutoff_2 = temp_2.int.uint.val.len() - precision_digits;
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

        Self {
            nb_steps,
            precision_bits,
            precision_digits,
            x,
            temp_1,
            temp_2,
            d,
            n,
            shift,
        }
    }

    fn run(&mut self) {
        let two = BigFloat::from(vec![T::ONE + T::ONE]);

        // Actual newton-raphson remaining steps
        for _ in 0..self.nb_steps {
            let cutoff_x = self.x.int.uint.val.len() - self.precision_digits;
            self.temp_1._set_to_mul(
                false,
                self.d.scale,
                &self.d.int.uint.val,
                self.x.int.sign,
                self.x.scale + (cutoff_x as isize),
                &self.x.int.uint.val[cutoff_x..],
            );
            self.temp_1 += &two;
            let cutoff_1 = self.temp_1.int.uint.val.len() - self.precision_digits;
            self.temp_2._set_to_mul(
                self.x.int.sign,
                self.x.scale + (cutoff_x as isize),
                &self.x.int.uint.val[cutoff_x..],
                self.temp_1.int.sign,
                self.temp_1.scale + (cutoff_1 as isize),
                &self.temp_1.int.uint.val[cutoff_1..],
            );
            self.x.copy_from(&self.temp_2);
        }
    }

    fn compute(mut self) -> BigUint<T> {
        // Finish off by computing the actual division
        self.x >>= self.shift;
        let cutoff = self.x.int.uint.val.len() - self.precision_digits;
        self.temp_1._set_to_mul(
            true,
            self.x.scale + (cutoff as isize),
            &self.x.int.uint.val[cutoff..],
            true,
            0,
            &self.n.val,
        );
        BigInt::from(self.temp_1).uint
    }
}

// TODO: difference between the two is greater than size of d ?
fn div(n: &BigUint<u32>, d: &BigUint<u32>) -> BigUint<u32> {
    let mut machine = NewtonRaphsonMachine::new(n, d);
    machine.run();
    machine.compute()
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use rand::distributions::Standard;
    use rand::prelude::Distribution;
    use rand::{thread_rng, Rng};

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

    /// Randomize some tests to compare the result with num-bigint
    #[cfg(feature = "rand")]
    fn coherence_with_num_bigint_random(n: usize, size_1: usize, size_2: usize) {
        assert!(size_1 < size_2);
        println!("STEP {n}");

        let vec_a = gen_n_random_values::<u32>(size_1);
        let vec_b = gen_n_random_values::<u32>(size_2);

        let a = BigUint::new(vec_a.clone());
        let b = BigUint::new(vec_b.clone());
        let c = b / a;
        let should_get = c.to_u32_digits();

        let biguinta = crate::BigUint::from(vec_a.clone());
        let biguintb = crate::BigUint::from(vec_b.clone());
        // let got_biguint = &biguintb / &biguinta;
        // let got_main = got_biguint.val;
        let got_newt_raph = super::div(&biguintb, &biguinta).val;

        if should_get != got_newt_raph {
            // assert_eq!(got_schoolbook, should_get);
            assert_eq!(should_get.len(), got_newt_raph.len());
            for (i, (a, b)) in should_get.iter().zip(got_newt_raph.iter()).enumerate() {
                if a > b {
                    println!("digit {i}, diff {}", a - b);
                } else if b > a {
                    println!("digit {i}, diff {}", b - a);
                }
            }
        }

        assert_eq!(should_get, got_newt_raph);
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test]
    #[cfg(feature = "rand")]
    fn coherence_with_num_bigint_many() {
        const SIZE0: usize = 100;
        const SIZE1: usize = 800;
        const SIZE2: usize = 1000;
        for n in 0..10 {
            let size_0 = SIZE0 + rand::thread_rng().gen_range(0..100);
            let size_1 = SIZE1 + rand::thread_rng().gen_range(0..100);
            let size_2 = SIZE2 + rand::thread_rng().gen_range(0..100);
            // coherence_with_num_bigint_random(n, size_0, size_2);
            coherence_with_num_bigint_random(n, size_1, size_2);
        }
    }
}
