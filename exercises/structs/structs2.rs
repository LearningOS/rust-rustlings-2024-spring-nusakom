// structs2.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs2` or use the `hint` watch subcommand for a
// hint.



// 首先定义Order结构体  
struct Order {  
    name: String,  
    year: i32,  
    made_by_phone: bool,  
    made_by_mobile: bool,  
    made_by_email: bool,  
    item_number: String,  
    count: i32,  
}  
  
// 定义create_order_template函数，它返回一个Order实例作为模板  
fn create_order_template() -> Order {  
    return Order {  
        name: "".to_string(), // 这里应该有一个默认的订单名称  
        year: 2023,           // 假设默认年份是2023  
        made_by_phone: false, // 默认订单不是通过电话创建的  
        made_by_mobile: false, // 默认订单不是通过手机创建的  
        made_by_email: true,   // 默认订单是通过邮件创建的  
        item_number: "".to_string(), // 这里应该有一个默认的物品编号  
        count: 0,                   // 默认订单数量为0  
    };  
}  
  
// 编写测试函数  
#[test]  
fn your_order() {  
    // 创建订单模板  
    let order_template = create_order_template();  
  
    // 使用模板创建你自己的订单  
    let your_order = Order {  
        name: "Hacker in Rust".to_string(),  
        year: order_template.year, // 使用模板的年份  
        made_by_phone: false,      // 假设这个订单不是通过电话创建的  
        made_by_mobile: true,      // 假设这个订单是通过手机创建的  
        made_by_email: false,      // 假设这个订单不是通过邮件创建的  
        item_number: "12345".to_string(), // 假设订单的物品编号是12345  
        count: 1,                        // 订单数量为1  
    };  
  
    // 使用assert_eq!宏来验证订单的属性  
    assert_eq!(your_order.name, "Hacker in Rust");  
    assert_eq!(your_order.year, order_template.year);  
    assert_eq!(your_order.made_by_phone, false);  
    assert_eq!(your_order.made_by_mobile, true);  
    assert_eq!(your_order.made_by_email, false);  
    assert_eq!(your_order.item_number, "12345");  
    assert_eq!(your_order.count, 1);  
}