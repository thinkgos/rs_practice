use anyhow::{ensure, Context, Result};

fn main() -> Result<()> {
    let a = 0;

    ensure!(a == 0, "a must be greater than zero");

    let content = std::fs::read("path")
        .with_context(|| format!("Failed to read instrs from {}", "path1111"))?;
    Ok(())
}
