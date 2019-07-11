use std::fmt;

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    println!("{} days", 31);

    println!("{0} this is {1}. {1} this is {0}.", "Alice", "Bob");

    println!("{hoh} {moh}",
             hoh = "moh",
             moh = "hoh");

    println!("{0} in binary {0:b}", 2);

    println!("{num:>0pad$}", num = 1, pad = 6);


    let s1 = Structure(900);
    let s2 = Structure(30);

    println!("{:p}", &s1);
    println!("{:p}", &s2);
    println!("{}", &s2);
    println!("{:?}", s1);
    println!("{:?}", Deep(Structure(800)));
    println!("=======");
    println!("{}", s1);
}
