pub mod md5sum;
pub mod sha256;

use std::path::PathBuf;

use base64::{engine::general_purpose, Engine};
use clap::{ArgAction, Args, ValueEnum};
use derive_more::Display;

#[derive(Args, Debug)]
pub struct Global {
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,
}

#[derive(Args, Debug)]
pub struct DigestFlag {
    /// 文本
    #[arg(short, long, group = "input")]
    pub text: Option<String>,
    /// 文件
    #[arg(short, long, group = "input")]
    pub file: Option<PathBuf>,
    /// format mode
    #[arg(short, long, default_value_t = Mode::Hex)]
    pub mode: Mode,
}

#[derive(Debug, Display, Clone, ValueEnum)]
pub enum Mode {
    /// hexadecimal format
    #[display(fmt = "hex")]
    Hex,
    /// base64 format
    #[display(fmt = "base64")]
    Base64,
    /// base64-url format
    #[display(fmt = "base64-url")]
    Base64Url,
}

impl Mode {
    fn format(&self, v: impl AsRef<[u8]>) -> String {
        match self {
            Mode::Hex => hex::encode(v),
            Mode::Base64 => general_purpose::STANDARD.encode(v),
            Mode::Base64Url => general_purpose::URL_SAFE_NO_PAD.encode(v),
        }
    }
}
