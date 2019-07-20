use std::fmt;
use std::str;

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Circle {{r={}}}", self.radius)
    }
}

impl str::FromStr for Circle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_result = s.parse::<i32>();
        let radius = match parse_result {
            Ok(r) => r,
            Err(_) => 0,
        };
        Result::Ok(Circle { radius })
    }
}

fn main() {
    let c = Circle { radius: 20 };
    println!("{}", c);

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    println!("{} + {} = {}", parsed, turbo_parsed, parsed + turbo_parsed);

    let c2 = "zoz".parse::<Circle>().unwrap();
    println!("{}", c2);

    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_squared + x_cube
    };

    let z = {
        2 * x
    };

    println!("{}, {}, {}", x, y, z);

    let xx =
        if 1 < 2 {
            1
        } else {
            2
        };

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);
}
