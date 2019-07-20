fn main() {
    fn increment(i: i32) -> i32 { i + 1 }

    let closure_annotated = |x: i32| -> i32 { x + 1 };
    let closure_inferred = |x| x + 1;

    let i = 1;

    println!("{}", increment(i));
    println!("{}", closure_annotated(i));
    println!("{}", closure_inferred(i));

    let one = || 1;
    println!("{}", one());
}