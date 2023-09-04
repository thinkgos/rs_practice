use flate2::read::GzDecoder;
use std::error;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn error::Error>> {
    let f = File::open("testdata/foo.gz")?;
    let mut s = String::new();
    let mut d = GzDecoder::new(f);

    d.read_to_string(&mut s)?;

    println!("{}", s);

    Ok(())
}
