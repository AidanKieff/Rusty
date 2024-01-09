use std::rc::Rc;
#[derive(Debug, Clone)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
impl List {

    fn create_list() -> List {
        List::Nil
    }

    fn add_number(&mut self, num: i32) {
        let new_list = List::Cons(num, Rc::new(self.clone()));
        *self = new_list;
    }


}

fn main() {
    let mut weirdo = List::create_list();
    println!("{:?}", weirdo);
    weirdo.add_number(45);
    println!("{:?}", weirdo); 
    weirdo.add_number(46);
    weirdo.add_number(47);
    weirdo.add_number(48);
    println!("{:?}", weirdo);

}
