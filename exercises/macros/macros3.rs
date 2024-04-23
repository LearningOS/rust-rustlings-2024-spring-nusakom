// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.



macro_rules! my_macro {
    ($var:expr) => {
        println!("This is my macro. The value of $var is: {}", $var);
    }
}

fn main() {
    my_macro!(5); // 输出: This is my macro. The value of $var is: 5
    my_macro!("hello"); // 输出: This is my macro. The value of $var is: hello
}
