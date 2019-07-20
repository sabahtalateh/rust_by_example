fn main() {
    for n in 1..=100 {
        print!("{} ", n);
    }
    println!("");

    let names = vec!["Bob", "Alice", "Eve"];

    // names.iter() borrows every value
    // name is a &str
    // collection can be reused after loop
    for name in names.iter() {
        match name {
            &"Eve" => println!("{} is a spy!!", name),
            _ => println!("Hello {}", name),
        }
    }
    println!("{:?}", &names);

    let mut names = vec!["Bob", "Alice", "Eve"];

    // iter_mut() allows to modify value
    for name in names.iter_mut() {
        *name = match name {
            &mut "Eve" => "Eve the spy!!",
            &mut x => x,
        }
    }
    println!("{:?}", names);

    let names = vec!["Bob", "Alice", "Eve"];
    // into_iter() moves value so it cannot be used after the loop
    for name in names.into_iter() {
        match name {
            "Eve" => println!("{} is a spy!!", name),
            _ => println!("Hello {}", name),
        }
    }
    // Error! Cannot use moved `names`
    // println!("{:?}", names);
}