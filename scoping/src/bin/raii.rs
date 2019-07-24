fn create_box() {
    let _box1 = Box::new(3i32);
}

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("Dropping..");
    }
}

fn main() {
    // Allocate an integer on the heap
    let _box2 = Box::new(5i32);

    // A nested scope:
    {
        // Allocate an integer on the heap
        let _box3 = Box::new(4i32);

        // `_box3` is destroyed here, and memory gets freed
    }

    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    for _ in 0u32..1_000 {
        create_box();
    }

    let _to_drop = ToDrop;
}
