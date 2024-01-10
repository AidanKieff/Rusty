fn main() {
    let s = Box::new("let's get it".to_owned());

    print(&s);

}

fn print(s: &str) {
    println!("{s}");
    
}