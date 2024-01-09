/*
fn main() {
     let s1: String = String::from("Rust");
     println!("{}", s1);
} //variable value assigned to variable s1 will be dropped after any scope that uses it
*/

/*
fn main() {
    let s1: String = String::from("Rust");
    let s2: String = s1;
    /* now can no longer call s1, because when creating s2
        we took the value s1 was assigned and moved it to s2
        and once this happens, the compiler frees s1 from being 
        anything in memory
     */
    println!("{}", s2); //not s1!
}
*/

//can clone 
fn main() {
    let s1: String = String::from("Rust");
    let s2: String = s1.clone();

    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s1);
    //note that integers are cloned by default, so x = 10 then y = x 
    //does not need y = x.clone() to compile and work successfully
    //y is automatically assigned a cloned value of x! 
}