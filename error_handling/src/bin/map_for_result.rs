use std::num::ParseIntError;

fn mul(a: &str, b: &str) -> Result<i32, ParseIntError> {
    match a.parse::<i32>() {
        Ok(a) => match b.parse::<i32>() {
            Ok(b) => Ok(a * b),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

fn mul2(a: &str, b: &str) -> Result<i32, ParseIntError> {
    a.parse::<i32>()
        .and_then(|a| b.parse::<i32>().map(|b| a * b))
}

fn print_result(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let x = mul("2", "ten");
    print_result(x);

    let x = mul2("3", "10");
    print_result(x);
}
