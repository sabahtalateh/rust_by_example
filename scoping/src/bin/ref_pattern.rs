#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let c = 'Q';

    // `ref` on the left side equal to `&` on the
    //  right side
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals to ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 3, y: 4 };

    // copy_of_x and copy_of_x2 is the same
    let copy_of_x = {
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;
        *ref_to_x
    };

    let copy_of_x2 = {
        let Point { x: ref_to_x, y: _ } = point;
        ref_to_x
    };

    println!("Copy of x is {:p}", &copy_of_x);
    println!("Copy of x is {:p}", &copy_of_x2);
    println!("Copy of x is {}", copy_of_x);

    // Mutable copy of point
    let mut mutable_point = point;

    {
        let Point {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mutable_point;
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!(
        "mutable_point is ({}, {})",
        mutable_point.x, mutable_point.y
    );

    // Mutable tuple
    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        // Desctructure `mutable_tuple` to change the value of the last element
        let (_, ref mut last) = mutable_tuple;
        *last = 4u32;
    }
    println!("{:?}", mutable_tuple);
}
