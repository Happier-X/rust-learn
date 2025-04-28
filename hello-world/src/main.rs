use std::io::stdin; //引入标准库io模块的stdin函数
// Prelude 会自动导入到每个Rust程序中，不需要显示的引入

// 项目入口，它总是先执行的函数，它可以在括号中传入参数
fn main() {
    let mut msg = String::new();
    println!("Please enter message:"); // 打印提示信息,调用了println!宏，!代表这不是一个普通的函数，而是一个宏
    stdin().read_line(&mut msg).unwrap();
    println!("Message is {}", msg);
}
