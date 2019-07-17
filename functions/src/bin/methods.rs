struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` methods go in here
impl Point {
    // This is a static method
    // Static methods don't need to be called by an instance
    // These methods are generally used as constructors
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another static method, taking two arguments:
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // &self is a sugar for self: &Self where &Self is Rectangle
    fn area(self: &Self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // Move self into destroy so it goes out of scope
    //  self is a sugar for self: Self
    fn destroy(self) {
        let Pair(left, right) = self;
        drop(left);
        drop(right);
    }
}

fn main() {
    let r = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle area is {}", r.area());
    println!("Rectangle perimeter is {}", r.perimeter());

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
}
