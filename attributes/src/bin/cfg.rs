#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are on linux");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* on linux");
}

fn main() {
    are_you_on_linux();
}
