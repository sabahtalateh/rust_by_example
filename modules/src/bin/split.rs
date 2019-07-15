// This declaration will look for a file named `my_mod.rs` or `my_mod/hello.rs` and will
//  insert its contents inside a module named `my` under this scope
mod my_mod;

fn function() {
    println!("call function()");
}

fn main() {
    // use lib;
    lib::hello::hello();
    lib::hello2::hello();

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
