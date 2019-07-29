// A unit struct without resources
#[derive(Debug, Clone, Copy)]
struct Nil;

// A tuple struct with resources that implements Clone trait
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    // Instanciate Nil
    let nil = Nil;

    // Copy Nil, there are no resources to move
    let copied_nil = nil;

    // Both nil's can be used independently
    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    // Instantiate `Pair`
    let pair = Pair(Box::new(12), Box::new(13));
    println!("original: {:?}", pair);

    // Copy `pair` into `moved_pair`, move resources
    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);

    // Can not use pair anymore as it was moved
    // println!("original: {:?}", pair);

    // Both Pairs can be used when use after clone
    let cloned_pair = moved_pair.clone();
    println!("cloned: {:?}", cloned_pair);
    println!("moved: {:?}", moved_pair);

    drop(moved_pair);

    // Can not use after drop
    // println!("moved: {:?}", moved_pair);

    // But cloned pair is still usable
    println!("cloned: {:?}", cloned_pair);
}
