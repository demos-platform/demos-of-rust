// 单文件
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub use crate::front_of_house::hosting;

// fn main() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();

//     println!("Hello, world!");
// }

// 多文件
// 在 mod front_of_house 后添加分号而不是代码块会让 Rust 前往与当前模块同名的文件中加载模块；
mod front_of_house;

pub use crate::front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    println!("Hello, world!");
}
