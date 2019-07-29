mod checked {
    // Mathematical "errors" we want to catch
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            // This operation would `fail`, instead let's return the reason of
            // the failure wrapped in `Err`
            Err(MathError::DivisionByZero)
        } else {
            // This operation is valid, return the result wrapped in `Ok`
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    pub fn op(x: f64, y: f64) -> MathResult {
        let div = div(x, y)?;
        let ln = ln(div)?;
        sqrt(ln)
    }
}

// `op(x, y)` === `sqrt(ln(x / y))`
fn op(x: f64, y: f64) {
    match checked::op(x, y) {
        Err(e) => {
            match e {
                checked::MathError::DivisionByZero => println!("division by zer"),
                checked::MathError::NonPositiveLogarithm => println!("non positive logarithm"),
                checked::MathError::NegativeSquareRoot => println!("negative square root"),
            }
        }
        Ok(x) => println!("{}", x),
    }
}

fn main() {
    op(1.0, 10.0);
}
