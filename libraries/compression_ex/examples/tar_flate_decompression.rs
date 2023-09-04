use flate2::read::GzDecoder;
use std::error;
use std::fs::File;
use tar::Archive;

fn main() -> Result<(), Box<dyn error::Error>> {
    let path = "testdata/archive.tar.gz";

    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    Ok(())
}
