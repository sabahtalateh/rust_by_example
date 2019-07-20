#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RGB(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

enum Foo1 {
    Bar,
    Baz,
    Qux(u32),
}

fn age() -> u32 {
    15
}

struct Foo {
    x: (u32, u32),
    y: u32,
}

fn main() {
    let n = 13;

    match n {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 | 13 => println!("Prime"),
        10...19 => println!("Teen"),
        _ => println!("Number.."),
    }

    let boolean = true;
    let binary = match boolean {
        true => 1,
        false => 0,
    };
    println!("{}", binary);

    let pair = (0, 2);

    match pair {
        (0, y) => println!("(zero, {})", y),
        (x, 0) => println!("({}, zero)", x),
        _ => println!("heh"),
    }

    let color = Color::RGB(0, 0, 0);
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("(r:{}, g:{}, b:{})", r, g, b),
        Color::CMYK(c, m, y, k) =>
            println!("cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                     c, m, y, k),
    }

    let reference = &4;

    match reference {
        // val contains exact value, not a reference
        &val => println!("{:?}", val),
    }

    // * before match is the same as previous variant
    match *reference {
        val => println!("{:?}", val),
    }

    let val = 5;
    match val {
        r => println!("[{:p}] {:?}", &r, r),
    }

    let mut mut_val = 6;
    // use ref in match to make reference from value
    match mut_val {
        // got a reference instead of simple value
        ref mut m => {
            *m += 90;
        }
    }
    println!("mut_val={}", mut_val);

    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("first of x is 1, second of x is {}, y is {}", b, y),
        Foo { x, y: 2 } => println!("y is 2, x is {:?}", x),
        Foo { y, .. } => println!("y is {}, don't care of x", y),
    }

    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("twins"),
        (x, y) if x + y == 0 => println!("kaboom"),
        _ => ()
    }

    match age() {
        0 => println!("Not born yet"),
        // to use n use @ to bind it
        n @ 1...12 => println!("Child of {} years", n),
        n @ 13...19 => println!("Teen of {} years", n),
        n => println!("Person {} years", n),
    }

    let number = Some(7);
    let letter: Option<i32> = None;
    let emotion: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}", i);
    }

    // Use else for failure
    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Did not match a number");
    }

    let i_like_letter = false;

    if let Some(i) = emotion {
        println!("Did not match a number");
    } else if i_like_letter {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    let a = Foo1::Bar;
    let b = Foo1::Baz;
    let c = Foo1::Qux(100);

    if let Foo1::Bar = a {
        println!("a is foobar");
    }

    if let Foo1::Bar = b {
        println!("b is foobar");
    }

    if let Foo1::Qux(n) = c {
        println!("c is fooqux({})", n);
    }

    let a = Foo1::Bar;
    if let Foo1::Bar = a {
        println!("a is foobar");
    }

    let mut optional = Some(0);
    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Grater than 9. quit");
                    optional = None;
                } else {
                    println!("i is {}. try again", i);
                    optional = Some(i + 1);
                }
            }
            None => break,
        }
    }

    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Grater than 9. quit");
            optional = None;
        } else {
            println!("i is {}. try again", i);
            optional = Some(i + 1);
        }
    }
}