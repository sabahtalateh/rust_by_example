use std::num::ParseIntError;

fn mul(a: &str, b: &str) -> i32 {
    let a = a.parse::<i32>().unwrap();
    let b = b.parse::<i32>().unwrap();
    a * b
}

// Main can return a Result
fn main() -> Result<(), ParseIntError> {
    let n1 = mul("2", "3");
    //    let n2 = mul("odin", "2");

    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number) => number,
        Err(err) => return Err(err),
    };
    println!("{}", number);
    Ok(())
}
