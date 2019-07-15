pub mod nested_mod;

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
