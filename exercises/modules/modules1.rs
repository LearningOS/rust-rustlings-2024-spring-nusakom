// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.



mod sausage_factory {
    // 私有函数，外部模块不能访问
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // 公有函数，外部模块可以访问
    pub fn make_sausage() {
        let recipe = get_secret_recipe(); // 在模块内部调用私有函数
        println!("按照秘方制作香肠! 秘方成分: {}", recipe);
        println!("香肠制作完成!");
    }
}

fn main() {
    sausage_factory::make_sausage(); // 正确调用模块中的公有函数
}
