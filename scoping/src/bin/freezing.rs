fn main() {
    let mut mutable_integer = 73;

    {
        let large_integer = 72;

        mutable_integer = 74;
        println!("{}", large_integer);
        println!("{}", mutable_integer);
    }
    mutable_integer = 90;
}
