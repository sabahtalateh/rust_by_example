use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn reverse_f32(pair: (f32, f32)) -> (f32, f32) {
    let (one, two) = pair;
    (two, one)
}

fn transpose(m: Matrix) -> Matrix {
    Matrix(m.0, m.2, m.1, m.3)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})\n", self.0, self.1)?;
        write!(f, "({}, {})", self.2, self.3)
    }
}

fn main() {
    let tuple = (1, 2, 3);
    println!("{}", tuple.0);

    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);

    let one_element_tuple = (1, );
    println!("one element tuple {:?}", one_element_tuple);

    let tuple = (1, 2, 3);
    let (a, b, c) = tuple;
    println!("{} {} {}", a, b, c);

    let matrix = Matrix(1.1f32, 1.2f32, 2.1f32, 2.2f32);
    println!("{:?}", matrix);
    println!("Matrix is \n{}", matrix);

    let transposed = transpose(matrix);
    println!("{}", transposed);
}
