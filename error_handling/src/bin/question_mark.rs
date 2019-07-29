// Question mark `?` return Err or extract value from result
// Before `?` was introduced the try! macro was used for the same purpose

use std::num::ParseIntError;

type MulResult<T> = Result<T, ParseIntError>;

fn mul_question(a: &str, b: &str) -> MulResult<i32> {
    // Same as
    // let a = match a.parse::<i32>() {
    //     Ok(n) => n,
    //     Err(e) => return Err(e),
    // };

    let a = a.parse::<i32>()?;
    let b = b.parse::<i32>()?;

    Ok(a * b)
}

//fn mul_try(a: &str, b: &str) -> MulResult<i32> {
//    let a = try!(a.parse::<i32>());
//    let b = try!(b.parse::<i32>());
//
//    Ok(a * b)
//}

fn print(x: MulResult<i32>) {
    match x {
        Ok(n) => println!("result is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let m = mul_question("11", "XII");
    print(m);

    let m = mul_question("11", "12");
    print(m);
}
