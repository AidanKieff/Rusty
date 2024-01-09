fn main() {
    // creation
    let a: i16 = 5;
    println!("a is: {a}");

    // mutability
    let mut b: i32 = 5;
    println!("b is: {b}");
    b =  10;
    println!("b is: {b}");
    // shadowing
    let c: i32 = 10;
    println!("c is: {c}");
    let c: i32 = 20;

    println!("c is: {c}");

    // scope
    let d: i32 = 30;
    {
        let d: i32 = 40;
        println!("inner d is: {d}");
    }

    println!("d is: {d}")

}
