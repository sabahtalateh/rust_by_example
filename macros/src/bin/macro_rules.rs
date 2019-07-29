macro_rules! say_hello {
    // () - indicates that macro takes no argument
    () => {
        // macro will expand to the content of the block
        println!("Hello")
    };
}

fn main() {
    say_hello!();
}
