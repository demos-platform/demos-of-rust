use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // 获取 cargo run 后面传入的参数。
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
