/*
borrow rules
at any given time, you can only have EITHER one mutable reference
or any number of immutable references

References must always be valid
*/

/*
fn main() {
    let mut s1: String = String::from("Rust");
    let r1: &String = &s1;
    // above s1 is immutably barrowed
    // if making the r2 variable right after, this would go against borrowing rules
    // however r1 is dropped after using the print string function
    // so we can use r1 theeeeen create r2 and rust will call that not breaking borrow rules
    // this is called nonlexical lifetime
    print_string(r1);
    let r2: &mut String = &mut s1;
    add_to_string(r2);
    println!("s1 is {s1}");
}

fn add_to_string(p1: &mut String) {
    p1.push_str(" is awesome");
}

fn print_string(p1: &String) {
    println!("{p1}");
}
*/

//if let

//instead of this:
/* 

let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    _ => (),
}
*/

//can do this:

let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}