use std::num::ParseIntError;

// Place Result into Option
fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    // Same as
    // match vec.first() {
    //     None => return None,
    //     Some(first) => {
    //         match first.parse::<i32>() {
    //             Ok(num) => Some(Ok(num)),
    //             Err(e) => Some(Err(e))
    //         }
    //     }
    // };
    vec.first().map(|first| first.parse::<i32>().map(|n| n * 2))
}

fn double_first_2(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));

    // Next lines are same as
    //
    // match opt {
    //     None => Ok(None),
    //     Some(x) => {
    //         match x {
    //             Err(e) => Err(e),
    //             Ok(n) => Ok(Some(n))
    //         }
    //     }
    // }

    let opt = opt.map_or(Ok(None), |r| r.map(Some));
    let opt = opt?;

    Ok(opt)
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["hello", "2", "3"];

    println!("first doubled {:?}", double_first(numbers));
    println!("first doubled {:?}", double_first(empty));
    println!("first doubled {:?}", double_first(strings));
    println!("");

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["hello", "2", "3"];

    println!("first doubled {:?}", double_first_2(numbers));
    println!("first doubled {:?}", double_first_2(empty));
    println!("first doubled {:?}", double_first_2(strings));
}
