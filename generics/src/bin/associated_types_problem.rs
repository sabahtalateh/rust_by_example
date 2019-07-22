struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, a: &A, b: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, a: &i32, b: &i32) -> bool {
        (&self.0 == a) && (&self.1 == b)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>,
{
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
