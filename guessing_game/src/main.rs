// 经验证标准库/第三方库 :: 相关方法支持智能提示。
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to guess game");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input guess number!");
        // 此处为什么要使用 mut：分析为该 guess 变量经过控制台输入后需要重新赋值。
        let mut guess = String::new();

        // 兜底机制
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // 为啥此处 guess 不需要 mut。分析为该 guess 变量在此处使用无需变更。
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
