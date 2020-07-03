#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 元组结构体（tuple structs）
struct Color(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // 不允许只标记某个变量可变,只能标准整个结构体可变
    // user1.email = String::from("anotheremail@example.com");

    // 简化赋值
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("{}", user1.active);
    println!("{:?}", user2);
}
