use flate2::write::GzEncoder;
use flate2::Compression;
use std::error;
use std::fs::File;

fn main() -> Result<(), Box<dyn error::Error>> {
    let f = File::create("testdata/archive.tar.gz")?;
    let enc = GzEncoder::new(f, Compression::default());
    let mut tar = tar::Builder::new(enc);

    tar.append_dir_all("src", "src")?;

    Ok(())
}
