// Rust infers a lifetime that is as short as possible
// The two references are the coerced to that lifetime
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` reads as lifetime of `a` at least as long as `b`
// We take in an `&'a i32` and return `&'b i32` as a result of coercion
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let a = 2; // Longer lifetime
    {
        let b = 3; // Shorter lifetime

        println!("The product is {}", multiply(&a, &b));
        println!("{} is the first", choose_first(&a, &b));
    }
}