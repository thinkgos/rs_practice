fn main() {
    // 所有权规则:
    // 1. Rust 中的每一个值都有一个被称为其所有者(owner)的变量.
    // 2. 值有且只有一个所有者.
    // 3. 当所有者(变量)离开作用域(scope),这个值将被丢弃.

    // 作用域(scope)
    println!("作用域:");
    let s = "hello";
    {
        // 内部s1作用域开始
        let s1 = "internal hello";
        println!("\tgot it internal s1 {}", s1);
    } // 内部s作用域结束
      // println!("\tgot it internal s1 {}", s1); // s1 超出作用域了
    println!("\tgot it {}", s);

    // 演示所有权的规则
    // string
    println!("所有权演示:");
    let strx = "hello"; // 这个是硬编码在程序里
    println!("\t硬编码: {}", strx);

    let mut str = String::from("hello"); // 动态内存(堆)申请的
    str.push_str(" world!");
    println!("\t动态编码: {}", str);

    // 所有权转移(move)
    println!("所有权move:");
    let str2 = str; // 浅拷贝,并使str无效. 称之为move.
                    // println!("\str已被转移: {}", str); 编译错误,str不再有对应内存的所有权,已被转移至str2
    println!("\t转移给str2: {}", str2);
    println!("clone深度复制数据:");
    // 可以采用深度复制数据,而不是转移所有权
    let str3 = str2.clone();
    println!("\t被clone数据: {}", str2);
    println!("\tclone数据: {}", str3);

    // NOTE: 栈上 Copy 操作,不涉及move
    // 所有整数类型,比如 u32.
    // 布尔类型bool,它的值是 true 和 false.
    // 所有浮点数类型,比如 f64.
    // 字符类型char.
    // 元组,当且仅当其包含的类型也都是 Copy 的时候
    //          比如,(i32, i32) 是Copy的
    //          但 (i32, String) 就不是.
    println!("栈上拷贝:");
    let x = 5;
    let y = x;
    println!("\tx = {}, y = {}", x, y);

    // 函数
    println!("所有权与函数:");
    let s = String::from("hello"); // s 进入作用域
    takes_ownership(s); // s 的值所有权移动到函数里
                        // println!("{}", s); 所以s到这里没有所有权,不再有效,

    let x = 5; // x 进入作用域
    makes_copy(x); // x 应该移动函数里,
    println!("\t函数外部仍可以操作数据: {}", x); // 但 i32 是 Copy 的,所以在后面可继续使用 x

    // 返回值与作用域
    println!("返回值与作用域:");
    // 变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它.
    // 当持有堆中数据值的变量离开作用域时,其值将通过 drop 被清理掉,除非数据被移动为另一个变量所有.
    let s1 = gives_ownership(); // gives_ownership 将返回值
    println!("\t获得函数返回值move的所有权数据: {}", s1); // 移给 s1

    let s2 = String::from("takes_and_gives_back"); // s2 进入作用域
    let s3 = takes_and_gives_back(s2); //
                                       // println!("{}", s2); // s2 被移动到 takes_and_gives_back 中,s2没有对应内存所有权
    println!(
        "\ts2数据move给函数内部,函数内部转移另一个所有权数据给s3: {}",
        s3
    ); // 它也将返回值移给 s3
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("\t函数内部获得数据所有权: {}", some_string);
} // 这里,some_string 移出作用域并调用 `drop` 方法.占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("\t函数内部获得数据的copy: {}", some_integer);
} // 这里,some_integer 移出作用域.不会有特殊操作

fn gives_ownership() -> String {
    // gives_ownership 将返回值移动给调用它的函数
    let some_string = String::from("gives_ownership"); // some_string 进入作用域.
    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}
