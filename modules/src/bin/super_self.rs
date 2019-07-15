fn function() {
    println!("call function()");
}

mod cool {
    pub fn function() {
        println!("call cool::function()");
    }
}

mod my {
    fn function() {
        println!("call my::function()");
    }

    mod cool {
        pub fn function() {
            println!("call my::cool::function()");
        }
    }

    pub fn indirect_call() {
        // Let's access all the functions named `function` from this scope!
        print!("call my::indirect_call(), that\n> ");

        // self::function and function will result the same
        self::function();
        print!("> ");
        function();

        // super refers to parent scope
        super::function();

        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
