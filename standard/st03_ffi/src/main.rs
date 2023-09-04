use std::ffi::{OsStr, OsString};

fn main() {
    let os_string = OsString::from("你好");
    println!("{:?}", os_string);
    let os_str = os_string.as_os_str();
    println!("{:?}", os_str);

    let s = os_string.into_string();
    if let Ok(v) = s {
        println!("{}", v)
    }

    let mut os_string = OsString::from("你好");
    os_string.push(", 世界");
    println!("{:?}", os_string);

    os_string.clear();
    assert_eq!(&os_string, "");

    let os_str = OsStr::new("hello");
    println!("{:?}", os_str);
}
