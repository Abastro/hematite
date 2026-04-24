#[macro_export]
macro_rules! the {
    ($first:expr, $($later:expr),*) => {
        {
            $(
            assert_eq!($first, $later);
            )*
            $first
        }
    }
}

#[macro_export]
macro_rules! the_debug {
    ($first:expr, $($later:expr),*) => {
        {
            $(
              debug_assert_eq!($first, $later);
            )*
            $first
        }
    };
}

/// Bit handling.
pub mod bits;

/// Mathematical basis, like prime finding.
pub mod math;