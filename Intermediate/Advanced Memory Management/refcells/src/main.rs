use std::rc::Rc;
use std::cell::RefCell;

struct Database {
    max_connections: u32,

}
/*
we use refcell and rc to get shared ownership (rc) and mutability (refcell methods) at the same time

*/
struct AuthService {
    db: Rc<RefCell<Database>>
}

struct ContentService {
    db: Rc<RefCell<Database>>
}

fn main() {
    let db = Rc::new(RefCell::new(Database {
        max_connections: 100
    }));
    let auth_service = AuthService {db: Rc::clone(&db)};
    let content_service = ContentService {db: Rc::clone(&db)};
    
    /* 
    the below shows something that will compile but panic at runtime
    this is because of ownership rulles (can't have two mutable references to data at the same time)
    the reason the compiler doesn't pick up on it is because the borrow_mut is an unsafe implementation called interrior mutability pattern

    so basically we use refcell to get around rusts ownership rules it goes through during compile time
    however, this means the developer has to ensure those ownership rules are followed during the run time of the program. the below is an example that is breaking ownership rules
    and will therefore panic when running. 

    let mut r1 = db.borrow_mut();
    let r2 = db.borrow_mut();
    
    r1.max_connections = 200;
    */

    db.borrow_mut().max_connections = 200;



}
