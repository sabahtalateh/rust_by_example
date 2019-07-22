use std::marker::PhantomData;
use std::ops::Add;

// Void enums to define unit types
#[derive(Debug, Copy, Clone)]
enum Inch {}
#[derive(Debug, Copy, Clone)]
enum Mm {}

/// Length is a type with phantom type parameter Unit
///  and not generic over the length type (f64)
#[derive(Debug, Copy, Clone)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Self) -> Self::Output {
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    // One foot have phantom data Inch
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    // One meter have phantom data Mm
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    // Since Length implements Copy, it will not be consumed
    // but copied into self and rhs
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // Addition works.
    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);
}
