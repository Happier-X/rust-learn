// fn main() {
//     let x = 5; // 语句
//     let y = {
//         let x = 3; // 表达式
//         x + 1 // 表达式，因为没有分号，它是一个表达式，所以它的值是 4
//     };
//     println!("The value of y is: {}", y);
//     println!("The value of x is: {}", x);
//     // 语句的值是 (), 表达式的值是 4
// }

// fn f(x: i32) -> i32 {
//     x + 1
// }
// fn main() {
//     println!(
//         "{}",
//         f({
//             let y = 1;
//             y + 1
//         })
//     )
// }

// fn main() {
//     let cond = true;
//     // let x = if cond { 1 } else { 2 };
//     // println!("x: {}", x);
//     let x;
//     if cond {
//         x = 1;
//     } else {
//         x = 2;
//     }
//     println!("x: {}", x);
// }

// fn main() {
//     let x = 1;
//     let y = if x { 0 } else { 1 };
//     println!("x: {}, y: {}", x, y);
// }

// fn main() {
//     let mut count = 0;
//     let result = loop {
//         count += 1;
//         if count == 10 {
//             break count * 2; // 返回值
//         }
//     };
//     println!("The result is: {result}"); // 20
// }

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;
//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break; // 退出内层循环
//             }
//             if count == 2 {
//                 break 'counting_up; // 退出外层循环
//             }
//             remaining -= 1;
//         }
//         count += 1;
//     }
//     println!("End count = {count}");
// }

// fn main() {
//     let mut x = 0;
//     'a: loop {
//         x += 1;
//         'b: loop {
//             if x > 10 {
//                 continue 'a; // 继续外层循环
//             } else {
//                 break 'b; // 退出内层循环
//             }
//         }
//         break; // 退出外层循环
//     }
//     println!("Final value of x: {}", x);
//     println!("Loop has ended.");
// }

fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("Liftoff!");
}
