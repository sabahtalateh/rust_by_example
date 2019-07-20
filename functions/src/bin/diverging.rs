// ! - type that never returns

fn main() {
//    let a = panic!("aa");
//    println!("Unreachable code");

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // return type of match is u32 but continue is not of u32 type
            //  but continue is of type ! that never returns so it is ok
            let addition = match i % 2 == 1 {
                true => i,
                false => continue,
            };
            acc += addition;
        }

        acc
    }
}

// ! is a type of endless loop, like web severs, or process termination functions like exit()
