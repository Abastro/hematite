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
