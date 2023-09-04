fn main() {
    // @@ 引用与借用
    // 1. 引用必须总是有效的. ---> 主要指悬垂引用
    // 2. 在任意给定时间 要么只能有一个可变引用,要么只能有多个不可变引用.
    // 4. 在特定作用域内 可以有多个可变引用(采用作用域隔开),但不能同时拥有.

    // 引用: & 符号就是引用,它们允许你使用值但不获取其所有权.
    // --> 不可变引用
    println!("借用 -> 不可变引用:");
    let s1 = String::from("hello");
    let len = unchange_borrow(&s1);
    println!("\tunchange borrow value '{}' length is {}.", s1, len);

    // --> 可变引用
    println!("借用 -> 可变引用:");
    // NOTE: 在特定作用域中的特定数据有且只有一个可变引用.
    let mut s = String::from("hello");
    change_borrow(&mut s);
    println!("\tchange borrow value: {}.", s);

    // --> 悬垂引用(Dangling References)
    println!("悬垂引用:");
    println!("\tline 48-59");
    // dangle()

    // slice  没有所有权类型. slice 允许你引用集合中一段连续的元素序列,而不用引用整个集合.
    println!("slice:");
    let mut s = String::from("hello world");

    let word = first_word(&s);
    // s.clear(); // 错误! 因为s已被不可变变量引用, 所以不可以clear
    println!("\tthe first word is: {}", word);
}

// 将获取引用作为函数参数称为 借用(borrowing),这里s就是借用.
fn unchange_borrow(s: &String) -> usize {
    // NOTE: 尝试修改借用的不可变变量是行不通的,因为不可变. 如果需要修改,需要采用可变引用.
    // some_string.push_str(", world");
    // s 是对 String 的引用
    s.len()
} // 这里,s 离开了作用域.但因为它并不拥有引用值的所有权,所以什么也不会发生

// 可变引用
fn change_borrow(some_string: &mut String) {
    some_string.push_str(", world");
}

// NOTE: 悬垂引用
// fn dangle() -> &String {
//     // dangle 返回一个字符串的引用
//     let s = String::from("hello"); // s 是一个新字符串
//
//     &s // 返回字符串 s 的引用
// } // 这里 s 离开作用域并被丢弃.其内存被释放.

// 采用所有权move,来避免 悬垂引用
// fn no_dangle() -> String {
//     String::from("hello")
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
