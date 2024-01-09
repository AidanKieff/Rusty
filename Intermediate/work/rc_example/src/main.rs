use std::rc::Rc;

// fn main() {
//     let player1 = Rc::new(String::from("player 1"));
//     let player2 = Rc::new(String::from("player 2"));

//     let result = first_turn(player1, player2);

//     print!("player going first is: {} \n", result);

// }

// fn first_turn(p1: Rc<String>, p2: Rc<String>) -> Rc<String> {
//     if rand::random() {
//         //Rc::clone(&p1)
//         p1
//     } else {
//         //Rc::clone(&p2)
//         p2
//     }
// }

// fn main() {
//     let player1 = Box::new(String::from("player 1"));
//     let player2 = Box::new(String::from("player 2"));

//     let result = first_turn(player1, player2);

//     print!("player going first is: {} \n", result);
//     // println!("{}", player1);
//     // println!("{}", player2);

// }

// fn first_turn(p1: &Box<String>, p2: &Box<String>) -> &Box<String> {
//     if rand::random() {
//         //Rc::clone(&p1)
//         &p1
//     } else {
//         //Rc::clone(&p2)
//         &p2
//     }
// }

fn main() {
    let player1 = Rc::new(String::from("player 1"));
    let player2 = Rc::new(String::from("player 2"));
    //let player3 = Rc::clone(&player1);

    let result = first_turn(Rc::clone(&player1), Rc::clone(&player2));

    print!("player going first is: {} \n", result);
    println!("{}", player1);
    println!("{}", player2);

}

fn first_turn(p1: Rc<String>, p2: Rc<String>) -> Rc<String> {
    if rand::random() {
        p1
    } else {
        p2
    }
}

// fn main() {
//     let player1 = String::from("player 1");
//     let player2 = String::from("player 2");

//     let result = first_turn(&player1, &player2);

//     print!("player going first is: {} \n", result);
//     println!("{}", player1);
//     println!("{}", player2);
    

// }

// fn first_turn(p1: &str, p2: &str) -> String {
//     if rand::random() {
//         p1.to_string()
//     } else {
//         p2.to_string()
//     }
// }