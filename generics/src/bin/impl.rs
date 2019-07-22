// Concrete type
struct S;

// Generic type
struct GenericVal<T>(T);

// Implementation with concrete type i32
impl GenericVal<i32> {}

// Implementation with concrete type S
impl GenericVal<S> {}

// Generic implementation
impl<T> GenericVal<T> {}

//
// Example
//

struct Val {
    val: f64,
}

struct GenVal<T> {
    val: T,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl<T> GenVal<T> {
    fn value(&mut self) -> &mut T {
        &mut self.val
    }
}

fn main() {
    let val = Val { val: 14 as f64 };
    println!("val.val = {}", val.value());

    let mut gen_val = GenVal {
        val: "Hello".to_string(),
    };
    let mut x = gen_val.value();
    (*x).push_str("!!");
    println!("gen_val.val = {}", gen_val.value());
}
