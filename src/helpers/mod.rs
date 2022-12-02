use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

pub fn read_input(path: impl AsRef<Path>) -> io::Result<BufReader<File>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}
