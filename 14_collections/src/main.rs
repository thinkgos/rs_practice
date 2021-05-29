use std::collections::HashMap;

fn main() {
    // 新建空的vector
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{}", v.get(1).unwrap());

    // 采用vec宏
    let v2 = vec![1, 2, 3, 4];

    // hash map
    let mut ss1 = HashMap::new();
    ss1.insert(String::from("one"), 1);
    ss1.insert(String::from("two"), 2);
    ss1.insert(String::from("three"), 3);
    ss1.entry(String::from("one")).or_insert(3);
    println!("ss1 = {:?}", ss1);

    //根据旧值来更新一个值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // Entry 的 or_insert 方法在键对应的值存在时就返回这个值的 Entry
        // 如果不存在则将参数作为新值插入并返回修改过的 Entry
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map = {:?}", map);
    println!("Hello, world!");
}
