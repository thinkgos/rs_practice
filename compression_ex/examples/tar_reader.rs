use std::error;
use std::fs::File;
use std::io::prelude::*;
use tar::Archive;

fn main() -> Result<(), Box<dyn error::Error>> {
    let file = File::open("testdata/foo.tar")?;
    let mut a = Archive::new(file);

    for file in a.entries()? {
        // Make sure there wasn't an I/O error
        let mut file = file?;

        // Inspect metadata about the file
        println!("file path: {:?}", file.header().path()?);
        println!("file size: {}", file.header().size()?);

        // files implement the Read trait
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        println!("file content: {}\n", s);
    }

    Ok(())
}
