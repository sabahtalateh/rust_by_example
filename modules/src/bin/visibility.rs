mod my_mod {
    fn private_function() {
        println!("call my_mod::private_function()");
    }

    pub fn function() {
        println!("call my_mod::function()");
    }

    // Functions can access it's module private items
    pub fn indirect_access() {
        print!("call my_mod::indirect_access() that ");
        private_function();
    }

    pub mod nested_mod {
        pub fn function() {
            println!("call my_mod::nested_mod::function()");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("call my_mod::nested_mod::private_function()");
        }

        // (in crate::my_mod) makes the function visible only within the
        //  specified crate
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("call my_mod::nested_mod::public_function_in_my_mod(), that\n> ");
            public_function_in_nested();
        }

        // (self) makes the function visible only within the module
        //  the same as leave the function private
        pub(self) fn public_function_in_nested() {
            println!("call my_mod::nested_mod::public_function_in_nested()");
        }

        // (super) makes it visible only in parent module
        pub(super) fn public_function_in_super_mod() {
            println!("call my_mod::nested_mod::public_function_in_super_mod()");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("call my_mod::call_public_function_in_my_mod() that\n> ");
        nested_mod::public_function_in_my_mod();
        print!("> ");
        nested_mod::public_function_in_super_mod();
    }

    pub(crate) fn public_function_in_crate() {
        println!("call my_mod::public_function_in_crate()");
    }

    mod private_nested_mod {
        #[allow(dead_code)]
        pub fn function() {
            println!("call my_mod::private_nested_mod::function()");
        }
    }
}


fn function() {
    println!("call function()");
}

fn main() {
    function();
    my_mod::function();

    my_mod::indirect_access();
    my_mod::nested_mod::function();
    my_mod::call_public_function_in_my_mod();

    my_mod::public_function_in_crate();

    // Error! Visibility restriction.
    // my_mod::nested_mod::public_function_in_my_mod();

    // Error! Cannot access private module.
    // my_mod::private_nested_mod::function();
}
