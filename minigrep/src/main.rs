use std::env;

fn main() {
    // 获取 cargo run 后面传入的参数。
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
