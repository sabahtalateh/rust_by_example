use std::borrow::ToOwned;

fn create_fn() -> Box<Fn()> {
    let text = "Fn".to_owned();
    let closure = move || println!("This is a {}", text);

    Box::new(closure)
}

fn create_fn_mut() -> Box<FnMut()> {
    let mut text = "FnMut".to_owned();
    let closure = move || {
        text.push_str("!!");
        println!("This is a {}", text)
    };

    Box::new(closure)
}

fn create_fn_once() -> Box<FnOnce()> {
    let mut text = "FnOnce".to_owned();
    let closure = move || {
        text.push_str("!!!");
        println!("This is a {}", text);
        drop(text)
    };

    Box::new(closure)
}

fn make_map() {

}

fn main() {
    let fn_closure = create_fn();
    fn_closure();

    let mut fn_mut_closure = create_fn_mut();
    fn_mut_closure();
    fn_mut_closure();

    let fn_once_closure = create_fn_once();
    fn_once_closure();
}
