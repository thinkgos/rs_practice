fn main() {
    let a = "world".to_string();
    let b = &a;
    let c = *b;
    println!("{}", c);
}
