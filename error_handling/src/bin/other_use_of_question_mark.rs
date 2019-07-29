use std::error;
use std::fmt;

// Change the alias to `Box<error::Error>`
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {
    fn description(&self) -> &str {
        "invalid first item to double"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked
        None
    }
}

// The same structure as before but rather than chain all `Results`
// and `Options` along, we `?` to get the inner value out immediately.
fn double_first(vec: Vec<&str>) -> Result<i32> {
    // Same as line below
    // let first = match vec.first() {
    //     Some(x) => x,
    //     None => return Err(From::from(EmptyVec)),
    // };
    let first = vec.first().ok_or(EmptyVec)?;

    // Same as line below
    // let parsed = match first.parse::<i32>() {
    //     Ok(x) => x,
    //     Err(e) => return Err(From::from(e)),
    // };
    let parsed = first.parse::<i32>()?;

    Ok(parsed * 2)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
