struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut point = Point {
        x: 0,
        y: 0,
        z: 0,
    };

    {
        let borrowed_point = &point;
        let one_more_borrowed_point = &point;

        println!("point coordinates ({}, {}, {})", &point.x, &borrowed_point.y, &one_more_borrowed_point.z);

        let mutable_borrowed_point = &mut point;
        mutable_borrowed_point.y = 2;

        println!("{}", borrowed_point.y);
    }
}