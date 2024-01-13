// Make the program error out with appropriate message where required.

fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("cannot divide by zero");
        // TODO: panic with msg `divide by zero error`
    }
    a / b
}

fn main() {
    let _res = div(23, 0);
}
