use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("Welcome to Bulls and Cows");
    // 生成一个1到10之间的随机数
    let secret_number = rand::thread_rng().gen_range(1..11);
    // 记录玩家猜的次数
    let mut attempts = 0;
    loop {
        println!("Please input a number: ");
        // 存储用户输入的数据
        let mut guess = String::new();
        // 读取用户的输入
        // 为了处理可能发生的错误，我们使用 expect 方法
        io::stdin()
            .read_line(&mut guess)
            .expect("Oops! Something goes wrong");
        // 去除空白字符
        // 并指定转换类型
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input valid number");
                continue;
            }
        };
        attempts += 1;

        if guess < 1 || guess > 10 {
            println!("Please input a number between 1 and 10");
            continue;
        }
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => {
                println!("too small!");

                if attempts > 5 {
                    println!("tips: you have tried {} times", attempts);
                }
            }

            Ordering::Greater => {
                println!("too big!");

                if attempts > 5 {
                    println!("tips: you have tried {} times", attempts);
                }
            }

            Ordering::Equal => {
                println!("Congratulation you're right!");

                println!("tips: you have tried {} times", attempts);

                break;
            }
        }
    }
}
