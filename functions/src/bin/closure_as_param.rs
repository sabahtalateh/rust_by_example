// Closure as input parameter must be annotated with one of
//  Fn - closure captures by reference (&T)
//  FnMut - closure captures by mutable reference (&mut ref)
//  FnOnce - closure captures by value (T)

fn apply<F>(f: F)
    where F: FnOnce()
{
    f();
}

fn apply_to_3<F>(f: F) -> i32
    where F: Fn(i32) -> i32
{
    f(3)
}

fn double_fn(x: i32) -> i32 {
    x * 2
}

fn main() {
    let greeting = "Hello";

    // A non-copy type. `to_owned` make owned data from borrowed one
    let mut farewell = "goodbue".to_owned();

    let diary = || {
        // `greeting` by reference, apply requires Fn()
        println!("I said {}", greeting);

        // `farewell` by mutable reference, apply requires FnMut()
        farewell.push_str("..");
        println!("Then I said {}", farewell);

        // `farewell` moved, apply required FnOnce()
        drop(farewell);
    };

    apply(diary);

    let double = |x| x * 2;
    let six = apply_to_3(double);
    println!("{}", six);

    let six = apply_to_3(double_fn);
    println!("{}", six);
}
