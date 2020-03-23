use std::fs::File;
use std::io::{Read, self};

pub fn read_file(path: &str) -> io::Result<String>{
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}