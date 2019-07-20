trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        println!("u8: {}", *self);
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        println!("String: {}", *self);
        format!("String: {}", *self)
    }
}

// Static dispatch
fn do_something<T: Foo>(x: T) {
    x.method();
}

// Dynamic dispatching
fn do_something_trait_object(x: &dyn Foo) {
    x.method();
}

fn main() {
    // Static dispatch
    let x = 5u8;
    let y = "Hello".to_string();

    do_something(x);
    do_something(y);

    // casting
    let x = 6u8;
    do_something_trait_object(&x as &Foo);

    // coercing
    let y = "Hello".to_string();
    do_something_trait_object(&y);

    let a = 12;
    let a_ref = &a;
    let a_ptr = a_ref as *const _;
    let a_ptr_num = a_ptr as usize;
    println!("{:p}, {:x}, {:?}", a_ref, a_ptr_num, a_ptr);
}
