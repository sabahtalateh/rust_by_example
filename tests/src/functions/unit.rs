pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("zero division");
    } else if a < b {
        panic!("divide result is zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

//    #[test]
//    fn test_bad_add() {
//        // This assert would fire and test will fail.
//        // Please note, that private functions can be tested too!
//        assert_eq!(bad_add(1, 2), 3);
//    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    #[should_panic(expected = "divide result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(bad_add(10, 20), 30);
    }
}
