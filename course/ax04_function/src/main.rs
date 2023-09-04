fn main() {
    // @@ 函数
    println!("函数:");
    // Rust 代码中的函数和变量名使用snake case规范风格
    println!("\tThe function return value: {}", five(10));

    // @@ 语句和表达式
    println!("语句和表达式:");
    let x = 5;
    // 上面的x和下面的x不是一个作用域
    let y = {
        // 以下是个表达式,不带分号,代表返回值
        let x = 3; // 语句
        x + 1 // 表达式
    };

    println!("\tThe value of x is: {}", x);
    println!("\tThe value of y is: {}", y);

    // @@ 无返回值的函数 ()
    println!("无返回值的函数 -> ()");
    empty();

    // @@ 永不返回的发散函数 !
    println!("永不返回的发散函数 -> !")
}

// 带有参数的函数,具有返回值的函数
fn five(x: i32) -> i32 {
    println!("\tThe function arg is: {}", x); // 带分号, 代表一条语句
    5 // 不带分号,代表返回值
}

fn four(x: i32) -> i32 {
    6
}

// 无返回值 ()
fn empty() -> () {}

// 永不返回的发散函数 !
fn forever() {
    loop {}
}
