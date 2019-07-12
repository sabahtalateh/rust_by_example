use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element = {}", slice[0]);
    println!("{} elements at all", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    let ys: [i32; 500] = [0; 500];

    println!("array occupied {} bytes", mem::size_of_val(&xs));

    analyze_slice(&xs);
    analyze_slice(&xs[1..3]);
}