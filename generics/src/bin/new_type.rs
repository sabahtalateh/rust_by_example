struct Years(i64);
struct Days(i64);

impl Years {
    fn to_days(&self) -> Days {
        Days(&self.0 * 365)
    }
}

impl Days {
    fn to_years(&self) -> Years {
        Years(&self.0 / 365)
    }
}

fn old_enough(years: Years) -> bool {
    return years.0 >= 18;
}

fn main() {
    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(age));
    println!("Old enough {}", old_enough(age_days.to_years()));
    //    println!("Old enough {}", old_enough(age_days));
}
