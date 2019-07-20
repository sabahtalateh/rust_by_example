fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Sum of all squared odds up to 1000");
    let upper = 1000;

    // Imperative approach
    let mut acc = 0;
    for n in 0.. {
        let squared = n * n;
        if squared >= upper {
            break;
        } else if is_odd(squared) {
            acc += squared;
        }
    }
    println!("Sum is {}", acc);

    let sum_of_squared_odd_numbers = (0..)
        .map(|n| n * n)
        .take_while(|&squared| squared < upper)
        .filter(|&squared| is_odd(squared))
        .fold(0, |acc, squared| acc + squared);
    println!("Sum is {}", sum_of_squared_odd_numbers);
}
