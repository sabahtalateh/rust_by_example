// One input reference that must live at least
//  as long as `print_one`
fn print_one<'a>(x: &'a i32) {
    println!("{}", x);
}

// Mutable references are possible with with lifetime
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Different lifetimes may be required then one for
//  for both variables in complex programs
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x = {}, y = {}", *x, *y);
}

// Returning references that was passed is acceptable
//  however the correct lifetimes should be passed
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

// Can not return a reference to data created inside the function
//  because it will be dropped as reach the end of function
//fn invalid_output<'a>() -> &'a String {
//    &String::from("Hello")
//}

fn main() {
    let mut a = 11 as i32;
    print_one(&a);

    add_one(&mut a);
    print_one(&a);

    let z = pass_x(&a, &14);
    println!("{}", z);

    print_multi(&a, &z);
}
