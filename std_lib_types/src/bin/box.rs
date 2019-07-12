use std::mem;

struct Point {
    x: f64,
    y: f64,
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    // Allocate on stack
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // Allocate on heap
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    // On stack
    let point: Point = origin();
    let rect: Rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 4.0 },
    };

    // Heap allocated rectangle
    let boxed_rect = Box::new(Rectangle {
        p1: origin(),
        p2: origin(),
    });

    // Function output can be boxed
    let boxed_point = Box::new(origin());

    // Double box
    let box_in_a_box = Box::new(boxed_origin());

    // Stack located values has different size
    println!("Point occupies {} bytes in stack", mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes in stack", mem::size_of_val(&rect));

    // Boxed (Heap located) values has a usize size
    println!("Boxed rect occupies {} bytes in stack", mem::size_of_val(&boxed_rect));
    println!("Boxed point occupies {} bytes in stack", mem::size_of_val(&boxed_point));
    println!("Box in box occupies {} bytes in stack", mem::size_of_val(&box_in_a_box));

    println!("Pointer size is {}", mem::size_of::<usize>());

    let unboxed_point = *boxed_point;
    println!("Unboxed point occupies {} bytes on stack", mem::size_of_val(&unboxed_point));
}
