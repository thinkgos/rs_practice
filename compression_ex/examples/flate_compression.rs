use flate2::write::GzEncoder;
use flate2::Compression;
use std::error;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn error::Error>> {
    let f = File::create("testdata/foo.gz")?;
    let mut enc = GzEncoder::new(f, Compression::default());

    enc.write_all(b"foo")?;
    enc.write_all(b"bar")?;
    enc.flush()?;

    Ok(())
}
