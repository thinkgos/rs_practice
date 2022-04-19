fn main() {
    // @@ 数据类型

    // --> 标量(scalar)类型代表一个单独的值.与其它语言一致.
    // isize 和 usize 类型依赖运行程序的计算机架构：
    //     64 位架构上它们是 64 位的
    //     32 位架构上它们是 32 位的.
    // Rust 中当不指定类型时,默认整型i32,浮点f64.
    // Rust 中 char 类型,单引号指定,不同于字符串使用双引号.
    // Rust 的 char 类型的大小为四个字节(four bytes),并代表了一个 Unicode 标量值

    // Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型.
    // 数值类型: i8 i16 i32 i64 i128 isize
    //          u8 u16 u32 u64 u128 usize
    // 浮点类型: f32 f64
    // 布尔类型: bool
    // 字符类型: char
    // 单元类型: 即 () ，其唯一的值也是 ()

    // --> 复合类型(Compound types)可以将多个值组合成一个类型.
    // Rust 有两个原生的复合类型: 元组(tuple)和数组(array)

    println!("元组:");
    // 元组: 固定长度,不可变,每个位子均有一个类型,且类型不必相同
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 可使用 '.'访问
    println!("\ttup: ({}, {}, {})", tup.0, tup.1, tup.2);
    // 可使用模式匹配(pattern matching)来解构(destructure)元组值
    let (x, y, z) = tup; // 解构
    println!("\ttup: ({}, {}, {})", x, y, z);

    println!("数组:");
    // 数组: 固定长度的,一旦声明,它们的长度不能增长或缩小.
    // 类型声明方式: [类型；长度]
    // 操作和其它语言一致
    let a = [1, 2, 3, 4, 5];
    let element = a[0];
    println!("\tThe value of element is: {}", element);
    // 在方括号中包含每个元素的类型,后跟分号,再后跟数组元素的数量.
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("\tThe value of element is: {:?}", a);
    // 如果你希望创建一个每个元素都相同的数组,[初始值;长度]
    // 可以在中括号内指定其初始值,后跟分号,再后跟数组的长度
    let a = [3; 5];
    println!("\tThe value of element is: {:?}", a);
}
