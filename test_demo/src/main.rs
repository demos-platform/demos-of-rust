fn main() {
    let five = Some(5);
    // 命中 Some 模式，返回结果为 6 的 Some 值。
    let six = plus_one(five);
    // 命中 None 模式。
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
