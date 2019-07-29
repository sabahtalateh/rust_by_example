use std::num::ParseIntError;

type MulResult<T> = Result<T, ParseIntError>;

fn mul2(a: &str, b: &str) -> MulResult<i32> {
    a.parse::<i32>()
        .and_then(|a| b.parse::<i32>().map(|b| a * b))
}

fn print_result(result: MulResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
