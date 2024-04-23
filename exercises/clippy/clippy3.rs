// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // 使用if let来处理None的情况
        println!("This will not be printed because my_option is None");
    }

    let my_arr = &[
        -1, -2, -3, // 添加了逗号
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear(); // 使用clear方法来清空向量
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 使用模式匹配来交换变量
    (value_a, value_b) = (value_b, value_a);

    println!("value a: {}; value b: {}", value_a, value_b);
}
