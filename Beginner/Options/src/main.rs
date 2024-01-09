struct FullName {
    first: String,
    middle: Option<String>,
    last: String,
}

fn get_alias(name: &str) -> Option<&str> {
    match name {
        "Bob" => Some("The Builder"),
        "Elvis" => Some("The King"),
        _=> None,
    }
}

fn main() {
    let alice = FullName {
        first: String::from("Alice"),
        middle: Some(String::from("Bob")),
        last: String::from("Smith")
    };

    let jon = FullName {
        first: String::from("Jon"),
        middle: None,
        last: String::from("Snow")
    };

    println!("Alice's middle name is{} {} {}", 
    alice.first,
    alice.middle.as_ref().map_or("", |m| &m[0..1]),
    alice.last);

    let optional_nickname = alice.middle.as_ref().and_then(|m| get_alias(&m));

    println!("Alic's middle name's nickname is {}", optional_nickname.unwrap_or("none found"));

    println!("Jon's middle name is {}", 
    match jon.middle {
        None => "No middle name",
        Some(ref x) => x,
    });
}