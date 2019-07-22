struct Null;
struct Empty;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    // drops self as long as T
    fn double_drop(self, _: T) {}
}

fn main() {
    let null = Null;
    let empty = Empty;

    null.double_drop(empty);

    //    null;
    //    empty;
}
