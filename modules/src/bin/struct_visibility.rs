mod my {
    // Public struct with public field
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // Public struct with private field
    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // Public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents }
        }
    }
}

fn main() {
    // Public structs can be constructed as usual
    let open_box = my::OpenBox { contents: "public information" };

    // and their fields can be normally accessed
    println!("{}", open_box.contents);

    // Error! Can not assign to a private field of a public struct
    // let closed_box = my::ClosedBox { contents: "private contents" };

    // Can be created via new()
    let _closed_box = my::ClosedBox::new("private data");
    println!("{:?}", _closed_box);

    // Error! Contents can not be accessed directly
    // println!("{}", _closed_box.contents);
}
