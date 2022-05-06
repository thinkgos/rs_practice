fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(&string1, string2.as_str());

    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}