// Make a constant with 'static lifetime
static NUM: i32 = 12;

// Return a reference to 'NUM' where its `'static`
//  lifetime is coerced to that or the input argument
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // Make a `string` literal and print it:
        let static_string: &str = "I'm in readonly memory";
        println!("static_string: {}", static_string);

        // When the reference goes out of scope the static_string can
        //  no longer be used. But the data - "I'm in readonly memory" remains in binary file
    }

    {
        // Make integer to use in coerce_static
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced static: {}", coerced_static);
        println!("address of coerced static is: {:p}", coerced_static);
    }

    println!("NUM is still accessible: {}", NUM);
    println!("address of NUM is {:p}", &NUM);
}
