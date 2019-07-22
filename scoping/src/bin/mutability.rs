#[derive(Copy, Clone, Debug)]
struct Book {
    // &'static is a reference to a string allocated in read-only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("Immutable borrow {} - {}", book.title, book.year);
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("Mutable borrow {} - {}", book.title, book.year);
}

fn main() {
    let immutable_book = Book {
        // string literals have type `&'static str`
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // Create mutable copy of `immutable_book`
    let mut mutable_book = immutable_book;

    // Immutably borrow an immutable Book
    borrow_book(&immutable_book);

    // Immutably borrow a mutable Book
    borrow_book(&mutable_book);

    // Borrow a mutable object as mutable
    new_edition(&mut mutable_book);

    // Error! Cannot borrow an immutable object as mutable
    // new_edition(&mut immutable_book);
}