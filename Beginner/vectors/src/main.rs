

fn main() {
    let mut v = Vec::new();
    v.push(String::from("hello"));
    v.push(String::from("world"));

    let v2 = vec![1,2,3];

    let s = &v[0];
    //let s = v.remove(0);

    let s = v.get(4);
    println!("{}", s.unwrap());
    if let Some(e) = s {
        println!("{e}");
    }

    // match s {
    //     Some(e) => println!("{e}"),
    //     None => panic!("nope")
    // }
    

    for s in &mut v{
        s.push_str("!");

    }

    for s in &v {
        println!("{s}");
    }

    let mut v3 = vec![];

    for s in v {
        v3.push(s);
    }
}


