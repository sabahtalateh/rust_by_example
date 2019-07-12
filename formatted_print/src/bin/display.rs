use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    img: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{real}+{img}i", real = self.real, img = self.img)
    }
}

fn main() {
    let min_max = MinMax(0, 1);

    println!("Compare structures:");
    println!("Display: {}", min_max);
    println!("Debug: {:?}", min_max);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point_2d = Point2D { x: 1.14, y: 10f64 };
    println!("Compare structures:");
    println!("Display: {}", point_2d);
    println!("Debug: {:?}", point_2d);

    let c = Complex {
        real: 12.4,
        img: 3.9,
    };
    println!("Display: {}", c);
    println!("Debug: {:?}", c);
}
