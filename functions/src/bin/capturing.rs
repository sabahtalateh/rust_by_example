fn main() {
    use std::mem;

    let color = "green";

    // Closure borrows color and leave it borrowed until
    //  leave a scope
    let print = || println!("color is {}", color);

    // Call the closure using the borrow
    print();
    print();

    let mut count = 0;

    // mut inc requires because it mutably borrows count
    let mut inc = || {
        count += 1;
        println!("count is {}", count);
    };

    inc();
    inc();

    let _reborrow = &mut count;
    *_reborrow = 12;


    // A non-copy type
    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();

    let haystack = vec![1, 2, 3];

    // `move` moves haystack into the closure
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&2));

    // Can not use haystack after move
    // To borrow haystack instead of move remove `move` keyword from closure's signature
    // println!("{:?}", haystack);
}