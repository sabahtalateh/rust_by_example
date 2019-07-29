// `min!` will calculate the minimum of any number of arguments.
macro_rules! find_min {
    // Base case:
    ($x:expr) => {$x};
    // In the following example, surrounding the matcher with $(...),+
    // will match one or more expression, separated by commas.
    ($x:expr, $($y:expr),+) => {
        std::cmp::min($x, find_min!($($y), +))
    }
}

fn main() {
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1, 2 - 9, 3));
}
