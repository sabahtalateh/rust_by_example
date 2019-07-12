enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn inspect(event: WebEvent) {
    use crate::WebEvent::*;

    match event {
        PageLoad => println!("page load."),
        PageUnload => println!("page unload."),
        KeyPress(c) => println!("pressed '{}'.", c),
        Paste(string) => println!("{} pasted.", string),
        Click { x, y } => println!("click at ({}, {})", x, y),
    }
}

fn main() {
    use crate::WebEvent::*;

    let pressed = KeyPress('x');
    let paste = Paste("Hello".to_owned());

    inspect(pressed);
    inspect(paste);

    println!("roses are 0x{:06x}", Color::Red as i32);
}
