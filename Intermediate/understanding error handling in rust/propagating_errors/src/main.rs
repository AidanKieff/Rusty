
use::std::{io::{self, Read}, fs::File};

fn main() {

    //let contents = read_file("example.txt");
    let contents = match read_file("example.txt") {
        Ok(c) => c,
        Err(error) => panic!("error reading file: {}", error)
    };

    find("this", contents.as_str());

    
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn find(query: &str, data: &str) {
    for i in data.lines() {
        if i.contains(query) {
            println!("{}", i.to_string());
        }
    }
}