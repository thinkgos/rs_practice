fn main() {
    // 不可变变量
    println!("不可变变量:");
    let x = 5;
    println!("\tThe value of x is: {}", x);
    // x = 6;  NOTE: 不可变变量,不可修改,抛出错误信息
    println!("\tThe value of x is: {}", x);

    // 可变变量, 加 mut 关键字
    println!("可变变量:");
    let mut y = 5;
    println!("\tThe value of y is: {}", y);
    y = 6; // NOTE: 可变变量,可修改
    println!("\tThe value of y is: {}", y);

    // 常量
    // 不允许对常量使用mut.
    // Rust 常量的命名规范是使用下划线分隔的大写字母单词，并且可以在数字字面值中插入下划线来提升可读性
    println!("常量:");
    const MAX_POINTS: u32 = 100_000;
    println!("\tThe value of MAX_POINTS is: {}", MAX_POINTS);

    // 隐藏, shadowing
    // 我们可以定义一个与之前变量同名的新变量，而新变量会 隐藏 之前的变量
    // 采用隐藏,变量类型可以不关心.
    // 但可变变量是确定类型的,是不可以赋值其它类型的
    println!("变量隐藏:");
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("\tThe value of z is: {}", z);
}
