// struct 结构体
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple structs 元组结构体
struct Color(i32, i32, i32);
// NOTE: unit-like structs
struct Point {}

fn main() {
    // struct 是一个自定义数据类型,允许你命名和包装多个相关的值,从而形成一个有意义的组合
    // 不允许只标记某个变量可变,只能标准整个结构体可变
    println!("struct:");
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        sign_in_count: 1,
        active: true,
    };
    // 不允许只标记某个变量可变,只能标准整个结构体可变
    // user1.email = String::from("anotheremail@example.com");
    println!("\t{:#?}", user1);

    let email = String::from("another@example.com");
    // 简化赋值
    let user2 = User {
        email, // 变量与字段同名时的字段初始化简写语法
        username: String::from("anotherusername567"),
        ..user1 // 使用结构体更新语法从其他实例创建实例
    };
    println!("\t{}", user1.active);
    println!("\t{:?}", user2);

    // tuple structs 使用没有命名字段的元组结构体来创建不同的类型
    println!("tuple struct:");
    let black = Color(1, 2, 3);
    println!("\t{} {} {}", black.0, black.1, black.2)
}
