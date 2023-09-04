use std::str::FromStr;

use strum::{EnumCount, EnumIter, EnumMessage, EnumString, IntoEnumIterator};

#[derive(Debug, EnumCount, EnumIter, EnumMessage, EnumString)]
enum Color {
    #[strum(message = "Is Red", detailed_message = "Is a red")]
    Red,
    #[strum(serialize = "b")]
    Blue,
    #[strum(serialize = "g", serialize = "green")]
    Green,
}

fn main() {
    // EnumCount enum的个数
    println!("Variants: {:?}", Color::COUNT);

    // EnumIter 和 IntoEnumIterator 支持 enum 的 iter
    for color in Color::iter() {
        println!("{:?}", color);
    }

    // EnumMessage 支持 帮助信息
    println!(
        "message: {:?}\ndetailed meaage: {:?}",
        Color::Red.get_message(),
        Color::Red.get_detailed_message()
    );

    //  EnumString 支持 从字符串解析
    println!(
        "{:?} - {:?}",
        Color::from_str("green"),
        Color::from_str("g")
    );
}
