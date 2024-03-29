//1. c语言的方式定义
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

//2. rust语言提倡的方式定义
#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
}

//3.也可以是不同类型
#[derive(Debug)]
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

//4. 经典用法
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Change(i32, i32, i32),
}
//等同于
//struct QuitMessage{}; //类单元结构体
//struct MoveMessage {
//  x: i32,
//  y: i32,
//}
//struct WriteMessage(String)
//struct ChangeMessage(i32, i32, i32)

//5. 枚举类型的方法以及match
impl Message {
    fn print(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move x = {}, y = {}", x, y),
            Message::Change(a, b, c) => println!("Change a = {}, b = {}, c = {}", a, b, c),
            Message::Write(s) => println!("Write = {}", s),
            _ => println!("Write"),
        }
    }
}

fn main() {
    let i1 = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let i2 = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("c语言方式:");
    println!("{:#?}", i1);
    println!("{:#?}", i2);

    let i1 = IpAddr2::V4(String::from("127.0.0.1"));
    let i2 = IpAddr2::V6(String::from("::1"));
    println!("rust语言方式:");
    println!("{:#?}", i1);
    println!("{:#?}", i2);

    let i1 = IpAddr3::V4(127, 0, 0, 1);
    let i2 = IpAddr3::V6(String::from("::1"));

    println!("枚举成员也可以是不同类型:");
    println!("{:#?}", i1);
    println!("{:#?}", i2);


    let quit = Message::Quit;
    quit.print();

    let mo = Message::Move { x: 10, y: 20 };
    mo.print();

    let wri = Message::Write(String::from("Hello"));
    wri.print();

    let change = Message::Change(1, 2, 3);
    change.print();
    println!("Hello, world!");
}
