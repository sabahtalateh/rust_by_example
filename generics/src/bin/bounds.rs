trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.length
    }
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}x{}]", self.length, self.width)
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    width: f64,
}

fn print_area<T: HasArea>(figure: T) {
    println!("area is {}", figure.area());
}

fn debug_it<T: std::fmt::Debug + std::fmt::Display>(x: &T) {
    println!("{:?}", x);
    println!("{}", x);
}

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T
where
    Option<T>: std::fmt::Debug,
{
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 2 as f64,
        length: 3 as f64,
    };

    print_area(rectangle);

    let rectangle = Rectangle {
        width: 2 as f64,
        length: 3 as f64,
    };

    debug_it(&rectangle);

    let vec = vec![1, 2, 3];
    vec.print_in_option();
}
