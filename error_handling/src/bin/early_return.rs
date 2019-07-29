use std::num::ParseIntError;

type MulResult<T> = Result<T, ParseIntError>;

fn mul(a: &str, b: &str) -> MulResult<i32> {
    let a = match a.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    let b = match b.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    Ok(a * b)
}

fn print(x: MulResult<i32>) {
    match x {
        Ok(n) => println!("result is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let m = mul("11", "XI");
    print(m);
}
