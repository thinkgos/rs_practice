/*!
 * https://course.rs/advance/formatted-output.html
 *  - {}适用于实现了std::fmt::Display特征的类型,用来以更优雅、更友好的方式格式化文本，例如展示给用户
 *  - {:?}适用于实现了std::fmt::Debug特征的类型，用于调试场景
 */

fn main() {
    println!("Hello"); // => "Hello"
    println!("Hello, {}!", "world"); // => "Hello, world!"
    println!("The number is {}", 1); // => "The number is 1"
    println!("{:?}", (3, 4)); // => "(3, 4)"
    println!("{} {}", 1, 2); // => "1 2"
    println!("{value}", value = 4); // => "4"
    println!("{:04}", 42); // => "0042" with leading zeros
    eprintln!("Error: Could not complete task"); // 输出到标准错误

    // 指定位置参数
    println!("{1}{0}", 1, 2); // =>21
    println!("{1}{}{0}{}", 1, 2); // => 2112

    // 带名称的变量
    // NOTE: 带名称的参数必须放在不带名称参数的后面
    println!("{argument}", argument = "test"); // => "test"
                                               // println!("{abc} {1}", abc = "def", 2); // 报错

    // 格式化参数
    let v = 3.14159261;
    // Display => 3.14
    println!("{:.2}", v);
    // Debug => 3.14
    println!("{:.2?}", v);

    // 进制
    // #号来控制数字的进制输出:
    // #b, 二进制
    // #o, 八进制
    // #x, 小写十六进制
    // #X, 大写十六进制
    // x, 不带前缀的小写十六进制
    // 二进制 => 0b11011!
    println!("{:#b}!", 27);
    // 八进制 => 0o33!
    println!("{:#o}!", 27);
    // 十进制 => 27!
    println!("{}!", 27);
    // 小写十六进制 => 0x1b!
    println!("{:#x}!", 27);
    // 大写十六进制 => 0x1B!
    println!("{:#X}!", 27);

    // 不带前缀的十六进制 => 1b!
    println!("{:x}!", 27);

    // 使用0填充二进制，宽度为10 => 0b00011011!
    println!("{:#010b}!", 27);
}
