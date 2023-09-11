use clap::{Parser, Subcommand};

use crate::command::DigestFlag;
use crate::command::Global;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(flatten)]
    pub global: Global,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
#[non_exhaustive] // 表明未来还有其它元素添加
pub enum Command {
    /// 打印或验证MD5(128-bit)校验值
    Md5sum(DigestFlag),
    // 打印或验证SHA256校验值
    Sha256(DigestFlag),
}
