use clap::Parser;
use num_cpus;
use std::{ffi::OsStr, path::Path};
use url::Url;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Operate {
    /// output file
    #[clap(short, long, default_value = "/tmp/aaa.png", validator = valid_filename)]
    output: String,
    /// url to capture
    #[clap(short, long, validator = valid_url)]
    url: String,
}

fn valid_filename(name: &str) -> Result<(), String> {
    let path = Path::new(name);

    let parent = path.parent().and_then(|p| p.is_dir().then(|| p));
    if parent.is_none() {
        return Err("路径不存在".into());
    }
    let ext = path
        .extension()
        .and_then(|p| OsStr::to_str(p))
        .and_then(|ext| {
            let ext = ext.to_lowercase();
            match ext.as_str() {
                "jpb" | "png" | "jpeg" => Some(ext),
                _ => None,
            }
        });
    if ext.is_none() {
        return Err("扩展名必须为 jpg, jpeg 或者 png".into());
    }
    return Ok(());
}

fn valid_url(url: &str) -> Result<(), String> {
    if Url::parse(url).is_ok() {
        return Ok(());
    }
    return Err("invalid url".into());
}

fn main() {
    println!("cup number: {}", num_cpus::get());
    println!("cup number: {}", num_cpus::get_physical());
    let opt = Operate::parse();
    println!("{:#?}", opt)
}
