
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct MutableData {
    value: i32,
}

fn main() {
    let shared_data = Rc::new(RefCell::new(MutableData{ value: 42}));

    println!("{:?}", shared_data.borrow());

    let another_owner = Rc::clone(&shared_data);

    shared_data.borrow_mut().value = 99;
    println!("modified data: {:?}", shared_data.borrow());

    another_owner.borrow_mut().value = 123;
    println!("Modified data by another owner: {:?}", another_owner.borrow());





}
