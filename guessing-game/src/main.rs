use rand::Rng; // 引入随机数生成器
use std::cmp::Ordering; // 引入比较器
use std::io; // 引入标准库 

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101); // 生成一个 1 到 100 的随机数
    // 使用 loop 实现一个无限循环
    loop {
        println!("Please enter your guess.");
        let mut guess = String::new(); // 使用 let 关键字声明变量，mut 表示可变变量，::new() 的 :: 表明 new 是 String 类型的关联函数
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // read_line 返回的是一个 Result 类型，它接收一个引用（&表示），使用 expect 方法处理错误，如果 Result 实例的值是 Err，expect 会导致程序崩溃，并输出错误信息，如果 Result 实例的值是 Ok 则会获取 Ok 中的值并原样返回
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // 如果解析失败，继续循环
        }; // trim() 去掉字符串两端的空格，parse() 将字符串转换为数字
        println!("You guessed: {}", guess); // 使用 {} 占位符输出变量的值
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // break 语句用于跳出循环
            }
        } // cmp() 方法比较两个值的大小，返回一个 Ordering 枚举值，match 是一个控制流分支结构，用于匹配模式并执行相应的代码块
    }
}
