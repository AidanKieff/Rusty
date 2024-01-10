use std::ops::{Deref, DerefMut};

struct MysmartPointer<T> {
    value: T
}

impl<T> MysmartPointer<T> {
    fn new(value: T) -> MysmartPointer<T> {
        MysmartPointer { value }
    }
}

impl<T> Deref for MysmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T> DerefMut for MysmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn main() {
    let s = MysmartPointer::new(Box::new("let's get it".to_owned()));

    print(&s);

}

fn print(s: &str) {
    println!("{s}");

}