// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!



fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {  
    // "blue" 是一个字符串字面量，可以传递给期望 &str 的函数  
    some_function_expecting_str_ref(&"blue");  
  
    // "red".to_string() 返回一个 String，可以传递给期望 String 的函数  
    some_function_expecting_string("red".to_string());  
  
    // String::from("hi") 返回一个 String，同样可以传递给期望 String 的函数  
    some_function_expecting_string(String::from("hi"));  
  
    // "rust is fun!".to_owned() 也会返回一个 String，因为它将 &str 转换为 String  
    some_function_expecting_string("rust is fun!".to_owned());  
  
    // "nice weather".into() 通常用于将类型转换为其他类型，这里它将 &str 转换为 String  
    some_function_expecting_string("nice weather".into());  
  
    // format!("Interpolation {}", "Station") 返回一个格式化后的 String  
    some_function_expecting_string(format!("Interpolation {}", "Station"));  
  
    // &String::from("abc")[0..1] 是一个 &str 切片引用，可以传递给期望 &str 的函数  
    some_function_expecting_str_ref(&String::from("abc")[0..1]);  
  
    // "  hello there ".trim() 移除字符串两端的空白，并返回一个新的 String  
    some_function_expecting_string("  hello there ".trim().to_string());  
  
    // "Happy Monday!".to_string().replace("Mon", "Tues") 返回一个新的 String，其中 "Mon" 被替换为 "Tues"  
    some_function_expecting_string("Happy Monday!".to_string().replace("Mon", "Tues"));  
  
    // "mY sHiFt KeY iS sTiCkY".to_lowercase() 返回一个新的 String，其中所有字符都被转换为小写  
    some_function_expecting_string("mY sHiFt KeY iS sTiCkY".to_lowercase());  
}
fn some_function_expecting_str_ref(s: &str) {  
    println!("{}", s);  
}  
  
fn some_function_expecting_string(s: String) {  
    println!("{}", s);  
}