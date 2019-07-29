// An integer division that doesn't `panic!`
fn check_division(dividend: i32, divisor: i32) -> Option<i32> {
    if 0 == divisor {
        None
    } else {
        Some(dividend / divisor)
    }
}

// This function handles a division that may not succeed
fn try_division(dividend: i32, divisor: i32) {
    match check_division(dividend, divisor) {
        Some(quotient) => println!("{} / {} = {}", dividend, divisor, quotient),
        None => println!("{} / {} failed", dividend, divisor),
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    // Equivalent
    let _none1: Option<i32> = None;
    let _none2 = None::<i32>;

    let optional_float = Some(12.2);

    // Unwrapping a `Some` variant will extract the value wrapped.
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

    // Unwrapping a `None` variant will `panic!`
    println!("{:?} unwraps to {:?}", _none1, _none1.unwrap());
}
