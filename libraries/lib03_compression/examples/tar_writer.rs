use std::error;
use std::fs::File;
use tar::Builder;

fn main() -> Result<(), Box<dyn error::Error>> {
    let file = File::create("testdata/foo.tar")?;
    let mut a = Builder::new(file);

    a.append_path("testdata/file1.txt")?;
    a.append_file("file2.txt", &mut File::open("testdata/file2.txt")?)?;

    Ok(())
}
