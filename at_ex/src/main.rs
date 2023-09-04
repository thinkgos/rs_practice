fn main() {
    let mut i = 42;
    let x = &mut i;

    change_i(x);
    println!("{}", *x);

    *x = 41;
    println!("{}", *x);
}

fn change_i(i: &mut i32) {
    *i = 3;
}
