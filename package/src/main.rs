use std::fs::File;

#[derive(Debug)] // 这样可以可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(s) => {
            println!("{:?}", s);
            25
        }
    }
}

fn main() {
    let f = File::open("hello.txt");
    if let true = f.is_err() {
        print!("{}", "111")
    }
}
