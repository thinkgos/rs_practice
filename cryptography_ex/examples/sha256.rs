use data_encoding::{HEXLOWER, HEXUPPER};
use ring::digest::{Context, SHA256};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut context = Context::new(&SHA256);

    context.update(b"admin");
    let d = context.finish();

    println!("{}", hex::encode_upper(d.as_ref()));
    println!("{}", hex::encode(d.as_ref()));
    println!("{}", HEXUPPER.encode(d.as_ref()));
    println!("{}", HEXLOWER.encode(d.as_ref()));
    Ok(())
}
