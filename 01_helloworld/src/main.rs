use crate::List::{Cons, Nil};
use std::rc::Rc;


fn main() {
    let a = Rc::new(Cons(5, Rc::new(Nil)));
    let d = a.clone();
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));


    println!("{}", Rc::strong_count(&a));

    println!("Hello, world! {{}}");
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}