#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// Unit struct
struct Nil;

struct Pair(i32, i32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(rect: &Rectangle) -> f32 {
    let width = (rect.p1.x - rect.p2.x).abs();
    let height = (rect.p1.y - rect.p2.y).abs();
    width * height
}

fn square(p: Point, side: f32) -> Rectangle {
    let Point { x, y } = p;
    let new_point = Point {
        x: x + side,
        y: y + side,
    };

    Rectangle {
        p1: p,
        p2: new_point,
    }
}

fn main() {
    let name = "Peter";
    let age = 27;
    let p = Person { name, age };

    println!("{:?}", p);

    let point = Point { x: 0.3, y: 0.4 };
    let new_point = Point { x: 0.1, ..point };
    println!("{:?}", new_point);

    // Destructure
    let Point { x: my_x, y: my_y } = new_point;
    println!("{} {}", my_x, my_y);

    let rect = Rectangle {
        p1: Point { x: my_x, y: my_y },
        p2: point,
    };

    let _nil = Nil;

    let pair = Pair(1, 2);

    // Destructure
    let Pair(one, two) = pair;

    println!("{} {}", one, two);

    let rect2 = Rectangle {
        p1: Point { x: 9.0, y: 0.0 },
        p2: Point { x: 1.0, y: 4.4 },
    };

    println!("{}", rect_area(&rect2));

    let p1 = Point { x: 0.0, y: 0.0 };
    let r = square(p1, 2.5);
    println!("{:?}", r);
}
