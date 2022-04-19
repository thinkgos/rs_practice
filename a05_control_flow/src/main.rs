fn main() {
    // @@ if 表达式
    println!("if 表达式:");
    let number = 3;
    // NOTE: 注意的是代码中的条件 必须 是 bool 值
    if number == 5 {
        println!("\tcondition was equal 5");
    } else if number == 100 {
        println!("\tcondition was equal 100");
    } else {
        println!("\tcondition was not equal 5 and 100");
    }
    // --> 在 let 语句中使用 if
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("\tThe value of number is: {}", number);

    // @@ 循环重复执行
    // Rust 有三种循环：loop、while 和 for
    println!("循环重复执行:");
    // --> loop
    let mut count = 0;
    loop {
        count += 1;
        if count > 5 {
            break;
        }
    }
    println!("\tloop -> break: {}", count);
    // --> 从loop返回
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("\tloop -> The result is {}", result);
    // --> while
    let mut number = 3;
    while number != 0 {
        println!("\twhile -> number: {}", number);
        number = number - 1;
    }
    println!("\twhile -> stop!!!");

    // --> for  遍历集合
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("\tfor -> the value is: {}", element);
    }

    // 倒计
    for number in (1..4).rev() {
        println!("\tfor -> {}!", number);
    }
    println!("\tfor -> LIFTOFF!!!");
}
