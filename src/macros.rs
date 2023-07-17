/// macro that allows easy construction of a BigUint from any type T for
/// which From<T> is implemented. Particularly useful for base 10 digit
/// string: `let uint = biguint!["123456789101112131415"];`
#[macro_export]
macro_rules! biguint {
    ( $( $x:expr ),* ) => {
        {
            $(
                BigUint::from($x)
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
