/// macro that allows easy construction of a BigUint from any type T for
/// which From<T> is implemented. Particularly useful for base 10 digit
/// string: `let uint = biguint!["123456789101112131415"];`
#[macro_export]
macro_rules! biguint {
    ( $( $x:expr ),* ) => {
        {
            $(
                BigUint::<T>::from($x)
            )*
        }
    };
}

/// macro that allows easy construction of a BigInt from any type T for
/// which From<T> is implemented. Particularly useful for base 10 digit
/// string: `let uint = biguint!["-123456789101112131415"];`
#[macro_export]
macro_rules! bigint {
    ( $( $x:expr ),* ) => {
        {
            $(
                BigInt::from($x)
            )*
        }
    };
}

/// macro that builds test function from a generic input function, running
/// it on available Digit types: u32, u64
#[macro_export]
macro_rules! test_functions {
    ( $($label:ident, $name32:ident, $name64:ident);* $(;)? ) => {
        $(
            #[test]
            fn $name32() {
                $label::<u32>();
            }
            #[test]
            fn $name64() {
                $label::<u64>();
            }
        )*
    }
}

/// macro that builds test function from a generic input function, running
/// it on available Digit types: u32, u64
#[macro_export]
macro_rules! test_panic_functions {
    ( $($label:ident, $name32:ident, $name64:ident);* $(;)? ) => {
        $(
            #[test]
            #[should_panic]
            fn $name32() {
                $label::<u32>();
            }
            #[test]
            #[should_panic]
            fn $name64() {
                $label::<u64>();
            }
        )*
    }
}
