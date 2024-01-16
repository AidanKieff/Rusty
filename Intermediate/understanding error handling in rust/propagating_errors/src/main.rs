
use::std::{io::{self, Read}, fs::File};

fn main() {

    let contents = read_file("example.txt");
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}