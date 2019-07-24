fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x = {}, y = {}", x, y);
}

// Function that has no arguments but has a lifetime
fn failed_borrow<'a>() {
    let _x = 12;

    // Error _y does not live long enough
    // let _y: &'a i32 = &_x;
    // `y` does not outlive the function
}

fn main() {
    // Create variables to be borrowed below
    let (four, nine) = (4, 9);

    // Borrows ('&') of both variables are passed into the function
    print_refs(&four, &nine);
    // Any input must outlive the borrower
    // Lifetimes of four and nine must be longer then print_refs

    failed_borrow();
}
