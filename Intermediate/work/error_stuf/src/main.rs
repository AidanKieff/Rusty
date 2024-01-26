use std::{error, fs};

fn main() {
    let pat = "oo";
    let f = parse_file("example.txt", pat);
    match f {
        Ok(x) => println!("{x}"),
        Err(e) => println!("{}", e)
    }
}

fn parse_file(filename: &str, pattern: &str) -> Result<i32, Box<dyn error::Error>> {
    let s = fs::read_to_string(filename)?;
    let i = find_num(&s, pattern)?;
    let x = i.parse()?;
    Ok(x)

}

fn find_num(text: &str, pattern: &str) -> Result<String, String> {
    if text.contains(pattern) {
        let start = text.find(pattern).unwrap();
        Ok(text[start..start + pattern.len()].to_string())
    } else {
        Err("could not match pattern".to_string())
    }
}

