use std::collections::HashMap;

fn main() {
    let mut scope = HashMap::new();
    scope.insert(String::from("a"), "b");
    scope.insert("c".to_string(), "d");

    for (k, v) in &scope {
        println!("{}:{}", k, v);
    }
    println!("{:?}", scope);
}
