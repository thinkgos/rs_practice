use std::convert::TryInto;
use std::rc::Rc;

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            match command {
                Command::Uppercase => {
                    output.push(string.to_uppercase());
                }
                Command::Trim => {
                    output.push(string.trim().to_string());
                }
                Command::Append(n) => {
                    let mut s = String::from(string);
                    for _ in 0..*n {
                        s.push_str("bar");
                    }
                    output.push(s);
                }
            }
        }
        output
    }
}
