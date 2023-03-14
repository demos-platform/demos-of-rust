// 实现一：确保此 largest 函数只会被那些实现了 PartialOrd 与 Copy trait 的类型所调用。
// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];
//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

// 扩展一，如果不希望 largest 函数只能使用那些实现了 Copy trait 的类型，也可以使用 Clone 来替换 T trait 约束中的 Copy。
// 但是从性能角度来看，如果 vec 动态数组中存放着类似 String 等存放在堆上的类型是，不断的 clone 相对实现一性能会差一些。
// fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
//     let mut largest = list[0].clone();

//     for item in list.iter() {
//         if item > &largest {
//             largest = item.clone();
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];
//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

// 扩展二，将返回类型从 T 更改为 &T，同时修改函数体为一个引用，就无需再使用 Copy 或者 Clone 来进行 trait 约束了。
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}