fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32)
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32)
}

fn main() {
    // Create boxed i32 and stacked i32
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // Ownership not taken so the content can be borrowed again
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // Take a reference to the data inside the box
        let _ref_to_i32: &i32 = &boxed_i32;

        eat_box_i32(boxed_i32);

        // _ref_to_i32 is a reference to boxed_i32 that was destroyed earlier so it can be used
        //        borrow_i32(_ref_to_i32);
    }
    //    eat_box_i32(boxed_i32);
}
