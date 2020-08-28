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
    Quarter,
    Qu(UsState),
    Default,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::Qu(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
        _ => 100, // 通匹配
    }
}

fn main() {
    println!("penny: {}", value_in_cents(Coin::Penny));
    value_in_cents(Coin::Qu(UsState::Alaska));
}
