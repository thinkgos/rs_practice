fn main() {
    // 函数
    // Rust 代码中的函数和变量名使用 snake case 规范风格
    let x = 5;

    // 以下是个表达式,不带分号,代表返回值
    // 上面的x和下面的x不是一个作用域
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    println!("The function return value: {}", five())
}

// 具有返回值的函数
fn five() -> i32 {
    // 不带分号,代表返回值
    // 带分号, 代表一条语句
    5
}
