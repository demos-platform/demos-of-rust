//【Todo】关于复杂列表章节的 3 道题目

// 1. 给定一组整数，使用动态数组来计算该组整数中的平均数、中位数（对数组进行排序后位于中间的值）及众数（出现次数最多的值；哈希映射可以在这里帮上忙）。

// 2. 将给定字符串转换为Pig Latin格式。在这个格式中，每个单词的第一个辅音字母会被移动到单词的结尾并增加“ay”后缀，例如“first”就会变为“irst-fay”。元音字母开头的单词则需要在结尾拼接上“hay”（例如，“apple”就会变为“apple-hay”）。要牢记我们讨论的关于UTF-8编码的内容！

// 3. 使用哈希映射和动态数组来创建一个添加雇员名字到公司部门的文本接口。例如，“添加Sally 至项目部门”或“添加Amir至销售部门”。除此之外，该文本接口还应该允许用户获得某个部门所有员工或公司中所有部门员工的列表，列表按照字母顺序进行排序。
use std::fmt::Display;
fn longest<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let string3 = String::from("测试");

    {
        let result = longest(string1.as_str(), string2.as_str(), string3);
        println!("The longest string is {}", result);
    }

}

fn test() -> String {
    let result = String::from("relly long string");
    result
}
