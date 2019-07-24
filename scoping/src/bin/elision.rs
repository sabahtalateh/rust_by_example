// `elided_input` and `annotated_input` essentially have identical signatures
// because the lifetime of `elided_input` is elided by the compiler:
fn elided_input(x: &i32) {
    println!("{}", x);
}

fn annotated_input<'a>(x: &'a i32) {
    println!("{}", x);
}

fn elided_pass(x: &i32) -> &i32 {
    x
}

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
    x
}

fn main() {
    let a = 12;
    elided_input(&a);
    annotated_input(&a);

    let _b = elided_pass(&a);
    let _b = annotated_pass(&a);
    println!("{}", _b);
}
