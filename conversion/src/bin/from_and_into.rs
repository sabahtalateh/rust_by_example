use std::convert;

#[derive(Debug)]
struct Number {
    n: i32,
}

impl convert::From<i32> for Number {
    fn from(n: i32) -> Self {
        Number { n }
    }
}

fn main() {
    let num1 = Number::from(12);
    println!("{:?}", num1);

    let num2: Number = 14.into();
    println!("{:?}", num2);
}
