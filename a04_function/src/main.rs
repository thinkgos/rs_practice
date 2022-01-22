fn main() {
    // 函数
    println!("函数:");
    // Rust 代码中的函数和变量名使用snake case规范风格
    println!("\tThe function return value: {}", five(10));

    // 语句和表达式
    println!("语句和表达式:");
    let x = 5;
    // 以下是个表达式,不带分号,代表返回值
    // 上面的x和下面的x不是一个作用域
    let y = {
        let x = 3;
        x + 1
    };

    println!("\tThe value of x is: {}", x);
    println!("\tThe value of y is: {}", y);
}

// 带有参数的函数,具有返回值的函数
fn five(x: i32) -> i32 {
    // 不带分号,代表返回值
    // 带分号, 代表一条语句
    println!("\tThe function arg is: {}", x);
    5
}

fn four(x: i32) -> i32 {
    6
}
