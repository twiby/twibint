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
