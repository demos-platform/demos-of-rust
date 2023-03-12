// 经验证标准库/第三方库 :: 相关方法支持智能提示。
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// Todo: 关于第 9 章提到的使用 Guess 结构体来实现校验输入数字的通用性。后续可做进一步尝试。
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Welcome to guess game");
    // Todo：待确认，上文第 4 行使用 use rand::Rng 后，为啥此处可以使用 rand::thread_rng().gen_range()
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

        // Todo: 使用 Guess 结构体优化此处，以达到校验数字校验的目的。
        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

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
