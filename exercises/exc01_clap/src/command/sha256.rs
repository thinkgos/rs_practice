use std::{fs::File, io::Read};

use anyhow::anyhow;
use ring::digest::{Context, SHA256};

use crate::command::DigestFlag;

pub fn run(flag: &DigestFlag) -> Result<(), anyhow::Error> {
    let mut context = Context::new(&SHA256);

    match (&flag.text, &flag.file) {
        (Some(text), _) => context.update(text.as_ref()),
        (None, Some(file)) => {
            let mut f = File::open(file)?;
            let mut buffer = [0; 4096]; // 定义一个缓冲区来处理字节流数据
            loop {
                let bytes_read = f.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }
                context.update(&buffer[..bytes_read]);
            }
        }
        _ => {
            return Err(anyhow!("text or file is required"));
        }
    }
    let digest = context.finish();

    println!("{}", flag.mode.format(&digest.as_ref()));
    Ok(())
}
