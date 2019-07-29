use std::env;

fn increase(number: i32) {
    println!("{}", number + 1);
}

fn decrease(number: i32) {
    println!("{}", number - 1);
}

fn help() {
    println!("usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one.");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let s = String::from("JJ J J");
//    let s1 = &s;
    let fw = first_word(&s[..]);
    println!("{}", fw);

    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[..]);

    let a = [1, 2, 3, 4, 5];
    let aa = &a[..];

    let a = vec![1, 2, 3];
    let aa = &a[..1];
    println!("{:?}", aa);

//    let args: Vec<String> = env::args().collect();
//
//    match args.len() {
//        1 => {
//            println!("My name is 'match_args'. Try passing some arguments!");
//        }
//        2 => {
//            match args[1].parse() {
//                Ok(42) => println!("This is the answer"),
//                _ => println!("Neh.."),
//            }
//        }
//        // one command and one argument passed
//        3 => {
//            let cmd = &args[1];
//            let num = &args[2];
//            // parse the number
//            let number: i32 = match num.parse() {
//                Ok(n) => n,
//                Err(_) => {
//                    eprintln!("error: second arg not an integer");
//                    help();
//                    return;
//                }
//            };
//            // parse the command
//            // &cmd[..] - makes
//            match &cmd[..] {
//                "increase" => increase(number),
//                "decrease" => decrease(number),
//                _ => {
//                    eprintln!("error: invalid command");
//                    help();
//                    return;
//                }
//            }
//        }
//        _ => help(),
//    }
}
