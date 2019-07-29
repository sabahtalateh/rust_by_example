use std::error;
use std::fmt;
use std::num;

// Change the alias to `Box<error::Error>`
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    // We will defer to the parse error implementation for their error
    // Supplying extra info requires adding more data to the type.
    Parse(num::ParseIntError),
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DoubleError::EmptyVec => write!(f, "please use vector with at least one element"),
            // This is a wrapper, so defer to the underlying types' implementation of `fmt`.
            DoubleError::Parse(e) => e.fmt(f),
        }
    }
}

impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            DoubleError::EmptyVec => None,
            DoubleError::Parse(e) => Some(e),
        }
    }
}

impl From<num::ParseIntError> for DoubleError {
    fn from(err: num::ParseIntError) -> Self {
        DoubleError::Parse(err)
    }
}

// The same structure as before but rather than chain all `Results`
// and `Options` along, we `?` to get the inner value out immediately.
fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
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
