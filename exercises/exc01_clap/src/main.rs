use anyhow::anyhow;
use clap::Parser;

use exc01::cli::{Cli, Command};
use exc01::command::{md5sum, sha256};

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    match &cli.command {
        Command::Md5sum(flag) => md5sum::run(flag)?,
        Command::Sha256(flag) => sha256::run(flag)?,
        _ => return Err(anyhow!("command not implemented")),
    }
    Ok(())
}
