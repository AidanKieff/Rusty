use std::{fs, io, error, num::ParseIntError};

fn main() {
    let i = parse_file("example.txt");
    match i {
        Ok(i) => println!("{i}"),
        Err(e) => {
            match e {
                ParseFileError::File => {
                    println!("something went wrong related to the file called");
                },
                ParseFileError::Parse(e) => {
                    println!("parse resulted in error: {}", e);
                }
            }
        }
    }

}

#[derive(Debug)]
enum ParseFileError {
    File, 
    Parse(ParseIntError)
}

fn parse_file(filename: &str) -> Result<i32, ParseFileError> {
    let s = fs::read_to_string(filename)
                        .map_err(|e| ParseFileError::File)?;
    let i = s.parse()
                        .map_err(|e| ParseFileError::Parse(e))?;
    Ok(i)
}