fn main() {
    // 标量（scalar）类型代表一个单独的值.与其它语言一致
    // Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。
    // i8 i16 i32 i64 i128 isize
    // u8 u16 u32 u64 u128 usize
    // f32 f64
    // bool
    // char

    // 复合类型（Compound types）可以将多个值组合成一个类型.
    // Rust 有两个原生的复合类型:元组（tuple）和数组（array）

    // 元组: 固定长度,不可变,每个位子均有一个类型,且类型不必相同
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // 解构
    println!("tup: ({}, {}, {})", x, y, z);
    // 可使用 '.'访问
    println!("tup: ({}, {}, {})", tup.0, tup.1, tup.2);

    // 数组: 固定长度的,一旦声明，它们的长度不能增长或缩小.
    // 操作和其它语言一致
    let a = [1, 2, 3, 4, 5];
    let element = a[0];

    println!("The value of element is: {}", element);
}
