fn main() {
    // 所有权规则
    // Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
    // 值有且只有一个所有者。
    // 当所有者（变量）离开作用域(scope)，这个值将被丢弃。

    // 作用域(scope)
    let s = "hello";
    {
        // 内部s1作用域开始
        let s1 = "internal hello";
        println!("got it internal s1 {}", s1);
    } // 内部s作用域结束
      // println!("got it internal s1 {}", s1); // 超出作用域了
    println!("got it {}", s);

    // 演示所有权的规则
    // string
    let strx = "hello"; // 这个是硬编码在程序里
    println!("{}", strx);

    let mut str = String::from("hello"); // 动态内存申请的

    str.push_str(" world!");

    println!("{}", str);

    // 所有权转移
    let str2 = str; // 浅拷贝,并使str无效. 称之为move.
                    // println!("{}", str); 编译错误,str不再有对应内存的所有权,已被转移至str2
    println!("{}", str2);

    // 栈上 Copy 操作,不涉及move
    // 所有整数类型，比如 u32。
    // 布尔类型，bool，它的值是 true 和 false。
    // 所有浮点数类型，比如 f64。
    // 字符类型，char。
    // 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。

    // 函数
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // println!("{}", s); 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，所以在后面可继续使用 x

    // 返回值与作用域
    // 变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。
    // 当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉,除非数据被移动为另一个变量所有。
    let s1 = gives_ownership(); // gives_ownership 将返回值
    println!("{}", s1); // 移给 s1

    let s2 = String::from("takes_and_gives_back"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); //
                                       // println!("{}", s2); // s2 被移动到 takes_and_gives_back 中,s2没有对应内存所有权
    println!("{}", s3); // 它也将返回值移给 s3

    // 引用与借用
    // 引用必须总是有效的。
    // 在特定作用域内不能同时拥有可变引用和不可变引用.
    // 在特定作用域内有且只有一个可变引用.
    // 在特定作用域内可以有多个可变引用.

    //  引用: & 符号就是 引用，它们允许你使用值但不获取其所有权。
    // 不可变引用
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // 可变引用
    // NOTE: 在特定作用域中的特定数据有且只有一个可变引用.
    let mut s = String::from("hello");

    change(&mut s);
    println!("{}.", s);

    // 悬垂引用（Dangling References）
    // set dangle()

    // slice  没有所有权类型. slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn gives_ownership() -> String {
    // gives_ownership 将返回值移动给
    // 调用它的函数

    let some_string = String::from("gives_ownership"); // some_string 进入作用域.

    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域

    a_string // 返回 a_string 并移出给调用的函数
}

// 将获取引用作为函数参数称为 借用（borrowing）,这里s就是借用.
fn calculate_length(s: &String) -> usize {
    // some_string.push_str(", world");  NOTE: 尝试修改借用的变量是行不通的,因为没有所有权. 如果需要修改,需要采用可变引用.
    // s 是对 String 的引用
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权,所以什么也不会发生

// 可变引用
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 悬垂引用
// fn dangle() -> &String {
//     // dangle 返回一个字符串的引用
//     let s = String::from("hello"); // s 是一个新字符串
//
//     &s // 返回字符串 s 的引用
// } // 这里 s 离开作用域并被丢弃。其内存被释放。

// 采用所有权move
// fn no_dangle() -> String {
//     let s = String::from("hello");
//
//     s
// }
