// NanoSecond is a new name for u64
type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;

fn main() {
    let nanosecond: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // Aliases are NOT new types
    println!("{} nanos + {} inches = {} crocodiles",
             nanosecond, inches, nanosecond + inches);
}
