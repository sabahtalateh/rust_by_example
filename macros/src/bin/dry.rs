use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
    ($a:ident, $b:ident, $func:ident, $op:tt) => {
        assert!(
            "{:?}: dimension mismatch: {:?} {:?} {:?}",
            stringify!($func),
            ($a.len(),),
            stringfy($op),
            ($b.len(),)
        );
    };
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, Outpur=T> + Copy>()
    };
}

fn main() {}
