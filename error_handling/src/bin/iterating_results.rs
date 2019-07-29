fn main() {
    let strings = vec!["zopa", "543", "1"];
    let possible_numbers: Vec<_> = strings.into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("{:?}", possible_numbers);

    // Fail entire operation with `.collect()`
    // Change return type
    let strings = vec!["zopa", "543", "1"];
    let possible_numbers: Result<Vec<_>, _> = strings.into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("{:?}", possible_numbers);

    // Collect all valid values and failures with `.partition()`
    // Change return type
    let strings = vec!["zopa", "543", "1"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings.into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("{:?} {:?}", numbers, errors);

    let strings = vec!["zopa", "543", "1"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings.into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();

    println!("Numbers: {:?} Errors: {:?}", numbers, errors);
}