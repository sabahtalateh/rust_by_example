fn double_first(vec: Vec<&str>) -> i32 {
    // Error 1
    let first = vec.first().unwrap();
    // Error 2
    2 * first.parse::<i32>().unwrap()
}

fn main() {
    let numbers = vec!["1", "2", "3"];
    //    let empty = vec![];
    let strings = vec!["ryba", "2", "14"];

    println!("The first doubled is {}", double_first(numbers));
    //    println!("The first doubled is {}", double_first(empty));
    println!("The first doubled is {}", double_first(strings));
}
