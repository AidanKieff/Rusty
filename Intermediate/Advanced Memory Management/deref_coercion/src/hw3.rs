// Complete the update_value function.

use core::fmt::Debug;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let owner1 = Rc::new(RefCell::new("Harry"));
    print_value(&owner1);
    let owner2 = Rc::clone(&owner1);
    update_value(&owner2, "Ron");
    print_value(&owner1);
    print_value(&owner2);
}

fn update_value<T>(owner: &Rc<RefCell<T>>, value: T) {}

fn print_value<T: Debug>(owner: &Rc<RefCell<T>>) {
    println!("{:?}", owner.borrow());
}
