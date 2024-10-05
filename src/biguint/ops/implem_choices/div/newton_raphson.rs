use crate::errors::DivisionError;
use crate::traits::Digit;
use crate::traits::DivisionResult;
use crate::BigFloat;
use crate::BigUint;

use super::multiplication_helper::SmartMul;

struct NewtonRaphsonMachine<'a, T: Digit> {
    x: BigFloat<T>,
    temp_1: BigFloat<T>,
    temp_2: BigFloat<T>,
    shift: usize,
    precision_bits: usize,
    precision_digits: usize,

    // the opposite of d shifted to be between 0.5 and 1
    d_mod: BigFloat<T>,
    n: &'a BigUint<T>,
    d: &'a BigUint<T>,
}

impl<'a, T: Digit> NewtonRaphsonMachine<'a, T> {
    fn new(n: &'a BigUint<T>, d: &'a BigUint<T>) -> Self {
        assert!(n >= d);

        let t2 = -BigFloat::from(BigUint::<T>::from(241u32)) >> 7;
        let t1 = BigFloat::from(BigUint::<T>::from(361u32)) >> 7;

        let precision_bits = n.nb_bits() - d.nb_bits();
        let precision_digits = precision_bits / T::NB_BITS + 2;

        // All needed allocations
        let shift = d.nb_bits();
        let mut d_mod = BigFloat::from(d.clone()) >> shift;
        d_mod.round_nb_digits(precision_digits);

        let mut x = BigFloat::default().with_capacity((precision_digits * 2 + 1) * T::NB_BITS);
        let temp_1 = BigFloat::default().with_capacity((precision_digits * 2 + 2) * T::NB_BITS);
        let temp_2 = BigFloat::default().with_capacity(
            (precision_digits * 2 + 2).max(precision_digits + d.val.len()) * T::NB_BITS,
        );

        // Initial "guess"
        x.set_to_mul(&d_mod, &t2);
        x += t1;

        Self {
            precision_bits,
            precision_digits,
            x,
            temp_1,
            temp_2,
            d_mod: -d_mod,
            n,
            d,
            shift,
        }
    }

    fn precision(&self) -> usize {
        if self.temp_1 == BigFloat::default() {
            return usize::MAX;
        }

        -(self.temp_1.scale * (T::NB_BITS as isize) + (self.temp_1.int.uint.nb_bits() as isize))
            as usize
    }

    fn run_newton_raphson_steps(&mut self) -> DivisionResult<()> {
        self.step_1();

        // compute theoretical number of steps
        // the actual number of steps might a little more (1 or 2)
        // because of the rounding
        let mut init_depth = self.precision();
        let mut nb_steps = 0;
        while init_depth < self.precision_bits {
            nb_steps += 1;
            init_depth <<= 1;
        }

        // Actual newton-raphson remaining steps
        let mut counter = 0;
        while self.precision() <= self.precision_bits {
            counter += 1;
            if counter > nb_steps * 2 {
                return Err(DivisionError::InfiniteNewtonRaphson);
            }

            self.step_1();
        }

        Ok(())
    }

    /// This is based on the formula X(n+1) = X(n) + X(n) * (1 - D*X(n))
    ///
    /// Using this version has an additional addition but reduces the size
    /// of the first buffer significantly, while allowing to measure the
    /// precision at each step.
    fn step_1(&mut self) {
        self.temp_1.smart_mul(&self.d_mod, &self.x);
        self.temp_1 += T::ONE;
        self.temp_1.round_nb_digits(self.precision_digits);
        self.temp_2.smart_mul(&self.x, &self.temp_1);
        self.temp_2.round_nb_digits(self.precision_digits);
        self.x.smart_add_assign(&self.temp_2);
        self.x.round_nb_digits(self.precision_digits);
    }

    /// This is based on the formula X(n+1) = X(n) * (2 - D * X(n))
    #[allow(unused)]
    fn step_2(&mut self) {
        self.temp_1.smart_mul(&self.d_mod, &self.x);
        self.temp_1 += T::TWO;
        self.temp_1.round_nb_digits(self.precision_digits);
        self.temp_2.smart_mul(&self.x, &self.temp_1);
        self.temp_2.round_nb_digits(self.precision_digits);
        // TODO: this copy could be avoided with a cyclic buffer
        self.x.copy_from(&self.temp_2);
    }

    /// Finish off by computing the actual division
    /// Final stage, after x has converge
    fn perform_division_with_remainder(mut self) -> (BigUint<T>, BigUint<T>) {
        self.x >>= self.shift;
        self.temp_1
            .smart_mul(&self.x, self.n.msd(self.precision_digits));

        self.temp_1.round();
        let scale = self.temp_1.scale.max(0) as usize;
        let mut quot = self.temp_1.int.uint << scale * T::NB_BITS;

        let mut rem = self.temp_2.int.uint;
        rem.set_to_mul(&quot, self.d);
        if &rem > self.n {
            quot -= T::ONE;
            rem -= self.d;
        }
        rem.rsub_assign(self.n);

        (quot, rem)
    }

    fn compute_div(mut self) -> DivisionResult<(BigUint<T>, BigUint<T>)> {
        self.run_newton_raphson_steps()?;
        Ok(self.perform_division_with_remainder())
    }

    /// Takes the relevent digits of an argument
    #[inline]
    #[allow(unused)]
    fn msd<N: SmartMul<T>>(&self, float: &'a N) -> N::MSDRep<'a> {
        float.msd(self.precision_digits)
    }
}

#[cfg(test)]
fn div<T: Digit>(n: &BigUint<T>, d: &BigUint<T>) -> DivisionResult<BigUint<T>> {
    Ok(rem_div(n, d)?.0)
}

pub(crate) fn rem_div<T: Digit>(
    n: &BigUint<T>,
    d: &BigUint<T>,
) -> DivisionResult<(BigUint<T>, BigUint<T>)> {
    let (q, r) = NewtonRaphsonMachine::new(n, d).compute_div()?;
    debug_assert!(&r < d);
    debug_assert_eq!(&((d * &q) + &r), n);
    Ok((q, r))
}

#[cfg(test)]
#[cfg(feature = "rand")]
mod tests {
    use crate::traits::Digit;
    use crate::Imported;
    use num_bigint::BigUint;
    use rand::distributions::Standard;
    use rand::prelude::Distribution;
    use rand::{thread_rng, Rng};
    use typed_test_gen::test_with;

    const SIZE0: usize = 100;
    const SIZE1: usize = 800;
    const SIZE2: usize = 1000;

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
        println!("coherence_with_num_bigint_random");

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
        let got_newt_raph = super::div(&biguintb, &biguinta).unwrap().val;

        assert_digits_equal(&should_get, &got_newt_raph);
    }

    fn coherence_with_num_bigint_exact(n: usize, size_1: usize, size_2: usize) {
        assert!(size_1 < size_2);
        println!("STEP {n}");
        println!("coherence_with_num_bigint_exact");
        let vec_a = gen_n_random_values::<u32>(size_1);
        let vec_c = gen_n_random_values::<u32>(size_2 - size_1);

        let a = BigUint::new(vec_a.clone());
        let c = BigUint::new(vec_c.clone());
        let b = &a * &c;
        let should_get = c.to_u32_digits();
        let vec_b = b.to_u32_digits();

        let biguinta = crate::BigUint::from(vec_a.clone());
        let biguintb = crate::BigUint::from(vec_b.clone());
        // let got_biguint = &biguintb / &biguinta;
        // let got_main = got_biguint.val;
        let got_newt_raph = super::div(&biguintb, &biguinta).unwrap().val;

        assert_digits_equal(&should_get, &got_newt_raph);
    }

    fn coherence_with_num_bigint_exact_p1(n: usize, size_1: usize, size_2: usize) {
        assert!(size_1 < size_2);
        println!("STEP {n}");
        println!("coherence_with_num_bigint_exact");
        let vec_a = gen_n_random_values::<u32>(size_1);
        let vec_c = gen_n_random_values::<u32>(size_2 - size_1);

        let a = BigUint::new(vec_a.clone()) + 1u32;
        let c = BigUint::new(vec_c.clone());
        let b = &a * &c;
        let should_get = c.to_u32_digits();
        let vec_b = b.to_u32_digits();

        let biguinta = crate::BigUint::from(vec_a.clone()) + 1;
        let biguintb = crate::BigUint::from(vec_b.clone());
        // let got_biguint = &biguintb / &biguinta;
        // let got_main = got_biguint.val;
        let got_newt_raph = super::div(&biguintb, &biguinta).unwrap().val;

        assert_digits_equal(&should_get, &got_newt_raph);
    }

    fn coherence_with_num_bigint_exact_m1(n: usize, size_1: usize, size_2: usize) {
        assert!(size_1 < size_2);
        println!("STEP {n}");
        println!("coherence_with_num_bigint_exact");
        let vec_a = gen_n_random_values::<u32>(size_1);
        let vec_c = gen_n_random_values::<u32>(size_2 - size_1);

        let a = BigUint::new(vec_a.clone()) - 1u32;
        let c = BigUint::new(vec_c.clone());
        let b = &a * &c;
        let should_get = c.to_u32_digits();
        let vec_b = b.to_u32_digits();

        let biguinta = crate::BigUint::from(vec_a.clone()) - 1;
        let biguintb = crate::BigUint::from(vec_b.clone());
        // let got_biguint = &biguintb / &biguinta;
        // let got_main = got_biguint.val;
        let got_newt_raph = super::div(&biguintb, &biguinta).unwrap().val;

        assert_digits_equal(&should_get, &got_newt_raph);
    }

    fn coherence_with_num_bigint_exact_scalar(n: usize, size: usize) {
        println!("STEP {n}");
        println!("coherence_with_num_bigint_exact_scalar");
        let vec_a = gen_n_random_values::<u32>(size);
        let vec_c = gen_n_random_values::<u32>(1);

        let a = BigUint::new(vec_a.clone());
        let c = BigUint::new(vec_c.clone());
        let b = &a * &c;
        let should_get = c.to_u32_digits();
        let vec_b = b.to_u32_digits();

        let biguinta = crate::BigUint::from(vec_a.clone());
        let biguintb = crate::BigUint::from(vec_b.clone());
        // let got_biguint = &biguintb / &biguinta;
        // let got_main = got_biguint.val;
        let got_newt_raph = super::div(&biguintb, &biguinta).unwrap().val;

        assert_digits_equal(&should_get, &got_newt_raph);
    }

    fn coherence_with_num_bigint_bi_scalar(n: usize) {
        println!("STEP {n}");
        println!("coherence_with_num_bigint_exact_scalar");
        let vec_a = gen_n_random_values::<u32>(1);

        let a = BigUint::new(vec_a.clone());
        let mut b = BigUint::new(vec_a.clone());
        b += 1u32;
        let c = &b / &a;
        let should_get = c.to_u32_digits();
        let vec_b = b.to_u32_digits();

        let biguinta = crate::BigUint::from(vec_a.clone());
        let biguintb = crate::BigUint::from(vec_b.clone());
        // let got_biguint = &biguintb / &biguinta;
        // let got_main = got_biguint.val;
        let got_newt_raph = super::div(&biguintb, &biguinta).unwrap().val;

        assert_digits_equal(&should_get, &got_newt_raph);
    }

    fn coherence_with_num_bigint_same(n: usize, size: usize) {
        println!("STEP {n}");
        println!("coherence_with_num_bigint_same");
        let vec_a = gen_n_random_values::<u32>(size);

        let biguinta = crate::BigUint::from(vec_a.clone());
        let biguintb = crate::BigUint::from(vec_a.clone());
        // let got_biguint = &biguintb / &biguinta;
        // let got_main = got_biguint.val;
        let got_newt_raph = super::div(&biguintb, &biguinta).unwrap().val;

        let should_get = vec![1];
        assert_digits_equal(&should_get, &got_newt_raph);
    }

    fn coherence_with_num_bigint_based<T: Digit>(n: usize, size_1: usize, size_2: usize) {
        println!("STEP {n}");
        println!("coherence_with_num_bigint_based");
        assert!(size_1 < size_2);
        let mut vec_a = vec![T::ZERO; size_1];
        let mut vec_b = vec![T::ZERO; size_2];
        *vec_a.last_mut().unwrap() = T::ONE;
        *vec_b.last_mut().unwrap() = T::ONE;

        let biguinta = crate::BigUint::from(vec_a.clone());
        let biguintb = crate::BigUint::from(vec_b.clone());
        // let got_biguint = &biguintb / &biguinta;
        // let got_main = got_biguint.val;
        let got_newt_raph = super::div(&biguintb, &biguinta).unwrap().val;

        let mut should_get = vec![T::ZERO; size_2 - size_1 + 1];
        *should_get.last_mut().unwrap() = T::ONE;
        assert_digits_equal(&should_get, &got_newt_raph);
    }

    fn coherence_with_num_bigint_9<T: Digit>(n: usize, size: usize) {
        println!("STEP {n}");
        println!("coherence_with_num_bigint_9");
        let vec_a = vec![T::MAX];
        let vec_b = vec![T::MAX; size];

        let biguinta = crate::BigUint::from(vec_a.clone());
        let mut biguintb = crate::BigUint::from(vec_b.clone());
        // let got_biguint = &biguintb / &biguinta;
        // let got_main = got_biguint.val;
        let got_newt_raph = super::div(&biguintb, &biguinta).unwrap().val;

        let should_get = vec![T::ONE; size];
        assert_digits_equal(&should_get, &got_newt_raph);

        biguintb += T::ONE;
        let got_newt_raph = super::div(&biguintb, &biguinta).unwrap().val;
        assert_digits_equal(&should_get, &got_newt_raph);
    }

    fn assert_digits_equal<T: Digit>(lhs: &Vec<T>, rhs: &Vec<T>) {
        if lhs != rhs {
            // assert_eq!(got_schoolbook, lhs);
            assert_eq!(lhs.len(), rhs.len());

            let should_get_int = crate::BigInt::from(lhs.clone());
            let newt_raph_int = crate::BigInt::from(rhs.clone());
            let error = (should_get_int - newt_raph_int).to_string();
            println!("Error: {}", error.to_string());
            // assert!(error == "0" || error == "-1");

            for (i, (a, b)) in lhs.iter().zip(rhs.iter()).enumerate() {
                if a > b {
                    println!("digit {i}, diff {}", *a - *b);
                } else if b > a {
                    println!("digit {i}, diff {}", *b - *a);
                }
            }
        }

        assert_eq!(lhs, rhs);
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test]
    #[ignore]
    #[cfg(feature = "rand")]
    fn coherence_with_num_bigint_many() {
        for n in 0..100 {
            let size_0 = SIZE0 + rand::thread_rng().gen_range(0..100);
            let size_1 = SIZE1 + rand::thread_rng().gen_range(0..100);
            let size_2 = SIZE2 + rand::thread_rng().gen_range(0..100);

            coherence_with_num_bigint_random(n, size_0, size_2);
            coherence_with_num_bigint_random(n, size_0, size_1);
            coherence_with_num_bigint_random(n, size_1, size_2);
        }
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test]
    #[ignore]
    #[cfg(feature = "rand")]
    fn coherence_with_num_bigint_0() {
        for n in 0..100 {
            let size_0 = SIZE0 + rand::thread_rng().gen_range(0..100);
            let size_1 = SIZE1 + rand::thread_rng().gen_range(0..100);
            let size_2 = SIZE2 + rand::thread_rng().gen_range(0..100);
            coherence_with_num_bigint_exact(n, size_0, size_2);
            coherence_with_num_bigint_exact(n, size_0, size_1);
            coherence_with_num_bigint_exact(n, size_1, size_2);
        }
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test]
    #[ignore]
    #[cfg(feature = "rand")]
    fn coherence_with_num_bigint_1() {
        for n in 0..100 {
            let size_0 = SIZE0 + rand::thread_rng().gen_range(0..100);
            let size_1 = SIZE1 + rand::thread_rng().gen_range(0..100);
            let size_2 = SIZE2 + rand::thread_rng().gen_range(0..100);
            coherence_with_num_bigint_exact_p1(n, size_0, size_2);
            coherence_with_num_bigint_exact_p1(n, size_0, size_1);
            coherence_with_num_bigint_exact_p1(n, size_1, size_2);
        }
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test]
    #[ignore]
    #[cfg(feature = "rand")]
    fn coherence_with_num_bigint_2() {
        for n in 0..100 {
            let size_0 = SIZE0 + rand::thread_rng().gen_range(0..100);
            let size_1 = SIZE1 + rand::thread_rng().gen_range(0..100);
            let size_2 = SIZE2 + rand::thread_rng().gen_range(0..100);
            coherence_with_num_bigint_exact_m1(n, size_0, size_2);
            coherence_with_num_bigint_exact_m1(n, size_0, size_1);
            coherence_with_num_bigint_exact_m1(n, size_1, size_2);
        }
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test]
    #[ignore]
    #[cfg(feature = "rand")]
    fn coherence_with_num_bigint_3() {
        for n in 0..100 {
            let size_0 = SIZE0 + rand::thread_rng().gen_range(0..100);
            let size_1 = SIZE1 + rand::thread_rng().gen_range(0..100);
            let size_2 = SIZE2 + rand::thread_rng().gen_range(0..100);
            coherence_with_num_bigint_exact_scalar(n, size_0);
            coherence_with_num_bigint_exact_scalar(n, size_1);
            coherence_with_num_bigint_exact_scalar(n, size_2);
        }
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test]
    #[ignore]
    fn coherence_with_num_bigint_4() {
        for n in 0..100 {
            let size_0 = SIZE0 + rand::thread_rng().gen_range(0..100);
            let size_1 = SIZE1 + rand::thread_rng().gen_range(0..100);
            let size_2 = SIZE2 + rand::thread_rng().gen_range(0..100);
            coherence_with_num_bigint_same(n, size_0);
            coherence_with_num_bigint_same(n, size_1);
            coherence_with_num_bigint_same(n, size_2);
        }
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test]
    #[ignore]
    fn coherence_with_num_bigint_7() {
        for n in 0..100 {
            coherence_with_num_bigint_bi_scalar(n);
        }
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test_with(u32, u64)]
    #[cfg(feature = "rand")]
    #[ignore]
    fn coherence_with_num_bigint_5<T: Digit>() {
        for n in 0..100 {
            let size_0 = SIZE0 + rand::thread_rng().gen_range(0..100);
            let size_1 = SIZE1 + rand::thread_rng().gen_range(0..100);
            let size_2 = SIZE2 + rand::thread_rng().gen_range(0..100);
            coherence_with_num_bigint_based::<T>(n, size_0, size_2);
            coherence_with_num_bigint_based::<T>(n, size_0, size_1);
            coherence_with_num_bigint_based::<T>(n, size_1, size_2);
        }
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test_with(u32, u64)]
    #[cfg(feature = "rand")]
    #[ignore]
    fn coherence_with_num_bigint_6<T: Digit>() {
        for n in 0..100 {
            let size_0 = SIZE0 + rand::thread_rng().gen_range(0..100);
            let size_1 = SIZE1 + rand::thread_rng().gen_range(0..100);
            let size_2 = SIZE2 + rand::thread_rng().gen_range(0..100);
            coherence_with_num_bigint_9::<T>(n, size_0);
            coherence_with_num_bigint_9::<T>(n, size_1);
            coherence_with_num_bigint_9::<T>(n, size_2);
        }
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test_with(u32, u64)]
    fn sqrt_2<T: Digit>() {
        let n = match Imported::<T>::read_from_file("src/export/test_files/sqrt_2_v1.tw").unwrap() {
            Imported::Uint(n) => n,
        };

        let ret = super::div(&n, &n).unwrap();
        assert_eq!(ret.to_string(), "1");

        let mut n2 = &n >> 1;
        let ret = super::div(&n, &n2).unwrap();
        assert_eq!(ret.to_string(), "2");

        n2 += T::ONE;
        let ret = super::div(&n, &n2).unwrap();
        assert_eq!(ret.to_string(), "1");
    }
}
