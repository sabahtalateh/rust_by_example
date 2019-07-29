struct Fibonacci {
    curr: i32,
    next: i32,
}

impl Iterator for Fibonacci {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

impl Default for Fibonacci {
    fn default() -> Self { Fibonacci { curr: 1, next: 1 } }
}

fn main() {
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("next in sequence is {:?}", sequence.next());
    println!("next in sequence is {:?}", sequence.next());
    println!("next in sequence is {:?}", sequence.next());
    println!("next in sequence is {:?}", sequence.next());

    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // The `take(n)` method reduces an `Iterator` to its first `n` terms.
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in Fibonacci::default().take(4) {
        println!("> {}", i);
    }

    // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in Fibonacci::default().skip(4).take(5) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    // The `iter` method produces an `Iterator` over an array/slice.
    println!("Iterate the following array {:?}", &array);
    for n in array.iter() {
        println!("> {}", n);
    }
}
