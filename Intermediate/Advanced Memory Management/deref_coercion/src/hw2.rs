// Make the code compile by implementing Deref & DerefMut for Wrapper.

use std::ops::{Deref, DerefMut};

struct Wrapper<T>(T);

impl<T> Deref for Wrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {&self.0}
}

impl<T> DerefMut for Wrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {&mut self.0}
}

fn main() {
    let mut my_str = Wrapper(String::from("Ferris"));
    //the variable is a struct, but because of the deref and derefMut traits, can run string methods right on it
    my_str.push_str(" the crab!!");
    my_str.pop();
    assert!(are_equal(&my_str, "Ferris the crab!"));
}

fn are_equal(a: &str, b: &str) -> bool {
    a == b
}
