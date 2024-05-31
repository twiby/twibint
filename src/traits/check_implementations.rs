use crate::traits::Digit;
use crate::BigInt;
use crate::BigUint;
use std::ops::BitAnd;
use std::ops::BitAndAssign;
use std::ops::BitOr;
use std::ops::BitOrAssign;
use std::ops::BitXor;
use std::ops::BitXorAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Rem;
use std::ops::RemAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use typed_test_gen::test_with;

use std::ops::Add;
use std::ops::AddAssign;

macro_rules! op_bound_def {
    ($trait_name: ident, $op: ident) => {
        #[allow(unused)]
        trait $trait_name<T: Digit>
        where
            Self: $op<T>,
            Self: $op<Self::Value>,
            Self: for<'a> $op<&'a T>,
            Self: for<'a> $op<&'a Self::Value>,
        {
            type Value;
        }
    };
}

macro_rules! op_bound_implem {
    ($trait_name: ident, $type: ident) => {
        impl<T: Digit> $trait_name<T> for $type<T> {
            type Value = $type<T>;
        }
    };
}
macro_rules! op_bound_ref_implem {
    ($trait_name: ident, $type: ident) => {
        impl<'a, T: Digit> $trait_name<T> for &'a $type<T> {
            type Value = $type<T>;
        }
    };
}
macro_rules! op_bound {
    (($trait_name: ident, $op_name: ident, $fun: ident), ($traitassign_name: ident, $opassign_name: ident, $funassign: ident)) => {
        op_bound_def!($trait_name, $op_name);
        op_bound_def!($traitassign_name, $opassign_name);
        op_bound_implem!($traitassign_name, BigUint);
        op_bound_implem!($trait_name, BigUint);
        op_bound_ref_implem!($trait_name, BigUint);
        op_bound_implem!($traitassign_name, BigInt);
        op_bound_implem!($trait_name, BigInt);
        op_bound_ref_implem!($trait_name, BigInt);

        /// This tests every combination of refrence/values for pairs of argument,
        /// and checks that the answer is always the same (doesn't check the value)
        #[test_with(u32, u64)]
        fn $fun<T: Digit>() {
            let a = BigUint::<T>::from(vec![T::MAX, T::MAX, T::MAX]);
            let b = BigUint::<T>::new(T::MAX);
            let literal = T::MAX;

            let mut results = vec![];
            results.push($op_name::$fun(&a, &b));
            results.push($op_name::$fun(&a, b.clone()));
            results.push($op_name::$fun(a.clone(), &b));
            results.push($op_name::$fun(a.clone(), b.clone()));
            results.push($op_name::$fun(&a, &literal));
            results.push($op_name::$fun(&a, literal.clone()));
            results.push($op_name::$fun(a.clone(), &literal));
            results.push($op_name::$fun(a.clone(), literal.clone()));

            let mut n = a.clone();
            $opassign_name::$funassign(&mut n, &b);
            results.push(n);
            let mut n = a.clone();
            $opassign_name::$funassign(&mut n, b);
            results.push(n);
            let mut n = a.clone();
            $opassign_name::$funassign(&mut n, &literal);
            results.push(n);
            let mut n = a.clone();
            $opassign_name::$funassign(&mut n, literal);
            results.push(n);

            for i in 1..results.len() {
                assert_eq!(results[0], results[i]);
            }

            let a = BigInt::<T>::from(vec![T::MAX, T::MAX, T::MAX]);
            let b = BigInt::<T>::from_unsigned(T::MAX);
            let literal = T::MAX;

            let mut results = vec![];
            results.push($op_name::$fun(&a, &b));
            results.push($op_name::$fun(&a, b.clone()));
            results.push($op_name::$fun(a.clone(), &b));
            results.push($op_name::$fun(a.clone(), b.clone()));
            results.push($op_name::$fun(&a, &literal));
            results.push($op_name::$fun(&a, literal.clone()));
            results.push($op_name::$fun(a.clone(), &literal));
            results.push($op_name::$fun(a.clone(), literal.clone()));

            let mut n = a.clone();
            $opassign_name::$funassign(&mut n, &b);
            results.push(n);
            let mut n = a.clone();
            $opassign_name::$funassign(&mut n, b);
            results.push(n);
            let mut n = a.clone();
            $opassign_name::$funassign(&mut n, &literal);
            results.push(n);
            let mut n = a.clone();
            $opassign_name::$funassign(&mut n, literal);
            results.push(n);

            for i in 1..results.len() {
                assert_eq!(results[0], results[i]);
            }
        }
    };
}

op_bound!(
    (AddImplementations, Add, add),
    (AddAssignImplementations, AddAssign, add_assign)
);
op_bound!(
    (SubImplementations, Sub, sub),
    (SubAssignImplementations, SubAssign, sub_assign)
);
op_bound!(
    (MulImplementations, Mul, mul),
    (MulAssignImplementations, MulAssign, mul_assign)
);
op_bound!(
    (DivImplementations, Div, div),
    (DivAssignImplementations, DivAssign, div_assign)
);
op_bound!(
    (RemImplementations, Rem, rem),
    (RemAssignImplementations, RemAssign, rem_assign)
);

op_bound!(
    (BitAndImplementations, BitAnd, bitand),
    (BitAndAssignImplementations, BitAndAssign, bitand_assign)
);
op_bound!(
    (BitOrImplementations, BitOr, bitor),
    (BitOrAssignImplementations, BitOrAssign, bitor_assign)
);
op_bound!(
    (BitXorImplementations, BitXor, bitxor),
    (BitXorAssignImplementations, BitXorAssign, bitxor_assign)
);
