// use std::rc::Rc;

// struct Database {}

// struct AuthService {
//     db: Rc<Database>
// }

// struct ContentService {
//     db: Rc<Database>
// }

// fn main() {
//     let db = Rc::new(Database {});
//     let auth_service = AuthService {db: Rc::clone(&db)};
//     let content_service = ContentService {db: Rc::clone(&db)};
// }

use std::rc::Rc;
#[derive(Debug, Clone)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let _first_list = Rc::new(Cons(10, Rc::new(Cons(20, Rc::new(Nil)))));

    println!("The count after creating _first_list is {}", Rc::strong_count(&_first_list));

    let _second_list = Cons(8, Rc::clone(&_first_list));

    println!("The count after creating _second_list is {}", Rc::strong_count(&_first_list));

    println!("the value of _second_list is {:?}", _second_list);

    {
        let _third_list = Cons(9, Rc::clone(&_first_list));
        println!("The count after creating _third_list is {}", Rc::strong_count(&_first_list));
        println!("the value of _third_list is {:?}", _third_list);
        println!("the value of __list is {:?}", _first_list);

    }

    
}