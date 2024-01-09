
/* when running a function that takes a variable as an argument, that 
    true input variables value is then owned by the argument variable of 
    the function. once the function is run, that arguemtn varable and it's value is 
    then dropped from memory. if you don't want the value dropped from the original
    variable assignment, you can either clone the original variable when
    using it as a function's argument, or make the function to where it only uses a reference 
    to the original variable and it's value. 
    
*/
fn main() {
    let s1 = String::from("Rust"); // heap allocated string
    let s2 = s1.clone();
    print_string(s1.clone());
    let s3 = generate_string();
    let s4 = add_to_string(s2);
    //in s4, s2 is passed to mut p1 then passed to s4
    //in the end s4 is not mutable so we can only print it out and not modify it again

    println!("s1 is: {s1}");
    println!("s1 is: {s1}");
    println!("s3 is {}", s3);
    println!("s4 is {s4}");

    let x = 10;
    let y = x;
    print_integer(x);
    println!("x is: {x}");
}

fn print_integer(i: i32) {
    println!(" i is {i}");
}

fn add_to_string(mut p1: String) -> String {
    p1.push_str(" is awesome");
    //because we are adding to the argument variable p1, 
    //it needs to be mutable
    // note that the variable used in the argument doesn't have to be mutable 
    // it will become mutable because it will pass ownership to the mutable p1
    p1
}

fn generate_string() -> String {
    String::from("Ferris")
}

fn print_string(p1: String) {
    println!("{p1}");
    println!("{p1}");
} // p1 is dropped, so best to assign a variable when calling this function??