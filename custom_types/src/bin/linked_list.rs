use crate::List::*;

#[derive(Debug)]
enum List {
    // Element and pointer to next node
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, val: u32) -> List {
        Cons(val, Box::new(self))
    }

    fn len(&self) -> u32 {
        match self {
            Cons(_, tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match self {
            Cons(head, tail) => {
                format!("{} {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    let mut list = List::new();

    list = list.prepend(19);
    list = list.prepend(20);
    list = list.prepend(29);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
    println!("{:?}", list);
}
