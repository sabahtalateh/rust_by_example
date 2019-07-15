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