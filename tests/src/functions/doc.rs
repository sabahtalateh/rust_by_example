/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `doccomments` crate:
/// ```
/// use tests::functions::doc;
/// let result = doc::add(1, 2);
/// assert_eq!(result, 3);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Usually doc comments may include sections "Examples", "Panics" and "Failures".
///
/// The next function divides two numbers.
///
/// # Examples
/// ```
/// use tests::functions::doc;
/// let result = doc::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// use tests::functions::doc;
/// doc::div(1, 0);
/// ```
///
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }

    a / b
}

/// Using hidden `try_main` in tests
///
/// ```
/// # // hidden lines starts with `#` but they are still compilable
/// # fn try_main() -> Result<(), String> { // line that wraps the body shown in doc
/// use tests::functions::doc;
/// let res = doc::try_div(10, 2)?;
/// # Ok(()) // return from try_main
/// # }
/// # fn main() { // starting main that'll unwrap()
/// #     try_main().unwrap() // calling try_main and unwrapping
/// #                         // so that test will panic in case of error
/// # }
/// ```
///
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("zero division".into())
    } else {
        Ok(a / b)
    }
}
