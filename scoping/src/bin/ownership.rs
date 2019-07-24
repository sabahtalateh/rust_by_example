/// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
    println!("destroying a box that contains {}", c);

    // `c` will be destroyed here
}

fn main() {
    // STACK allocated integer
    let x = 5u32;

    // *Copy* `x` into `y`. No resources are moved
    let y = x;

    // Both values can be independently used
    println!("x is {} and y is {}", x, y);

    // `a` is a pointer to HEAP allocated memory
    let a = Box::new(5i32);

    println!("a contains {}", a);

    // *Move* `a` into `b`
    let b = a;
    // The pointer address for `a` is copied (not the data) into `b`
    // Both are now points to the same heap allocated data
    // but `b` only owns it.

    // Error! a can no longer access the data because it is no longer owns
    // the heap memory
    // println!("a contains {}", a);

    // This function takes ownership of the heap allocated memory for `b`
    destroy_box(b);

    // Since the heap memory has been freed at this point, this action
    // would result in dereferencing free memory, but it's forbidden by the compiler
    // println!("b contains {}", b);

    //
    // Mutability can be changed when transfer ownership
    //
    let immutable_box = Box::new(4u32);
    println!("immutable_box {}", immutable_box);

    // immutable_box is immutable so it can not be changed
    // *immutable_box = 5;

    // *Move* the box. Changing ownership
    let mut mutable_box = immutable_box;

    println!("mutable_box {}", mutable_box);

    *mutable_box = 101;

    println!("mutable_box {}", mutable_box);
}
