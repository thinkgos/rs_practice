use std::convert::TryInto;
use std::rc::Rc;

fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    if a < (b as i32) {
        println!("Ten is less than one hundred.");
    }

    let a: u8 = 10;
    let b: u16 = 1500;

    let a_: u16 = a.try_into().unwrap();

    if a_ < b {
        println!("Ten is less than one hundred.");
    }
}