struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    {
        let borrowed_point = &point;
        let one_more_borrowed_point = &point;

        println!(
            "point coordinates ({}, {}, {})",
            &point.x, &borrowed_point.y, &one_more_borrowed_point.z
        );

        let mutable_borrowed_point = &mut point;
        mutable_borrowed_point.y = 2;

        println!("{}", mutable_borrowed_point.y);
        // Can not use borrowed point here as mutable borrow occurs earlier
        //         println!("{}", borrowed_point.x);
    }

    {
        let mutable_borrow = &mut point;
        mutable_borrow.x = 101;

        let x = &point.x;
        println!("{}", x);
    }

    // Can borrow immutable again
    let borrowed_point = &point;
    println!(
        "Point now has coordinates: ({}, {}, {})",
        borrowed_point.x, borrowed_point.y, borrowed_point.z
    );
}
