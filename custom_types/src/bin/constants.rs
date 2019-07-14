static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 5;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;

    let unit = ();
    println!("unit: {:?}", unit);

    let mut a = 2;
    {
        a = 3;
        println!("a {}", a);
    }
    println!("a {}", a);

    let b;

//    println!("{}", b);
    b = 2;
}