struct Container(i32, i32);

trait Contains {
    type A;
    type B;

    fn contains(&self, a: &Self::A, b: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, a: &Self::A, b: &Self::B) -> bool {
        (&self.0 == a) && (&self.1 == b)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.first() - container.last()
}

fn main() {
    let n1 = 3;
    let n2 = 10;

    let container = Container(n1, n2);

    println!(
        "Does the container contains {} and {}: {}",
        &n1,
        &n2,
        container.contains(&n1, &n2)
    );
    println!("First is {}", container.first());
    println!("Last is {}", container.last());
    println!("The difference is {}", difference(&container));
}
