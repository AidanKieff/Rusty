fn main() {
    // if/else
    let a: i32 = 5;
    //conditions in if/else statements must evaluate to boolian expressions
    if a > 5 {
        println!("bigger than 5");
    } else if a > 3 {
        println!("bigger than 3");
    } else {
        println!("smaller or equal to 3");
    }

    // If/else statements cn also be used inside let statements
    let _b: i32 = if a > 5 {1} else {-1};
    // this above let statement has if/else statements that
    // determine the value of b depending on if a is greater than 5 or not
    // return types must be the same! 
    // underscored b because not actually using it. if I add say a println
    // then I wouldn't underscore it!


    // loop 
    loop {
        println!("loop forever unless you have a break statement");
        break;
    }

    // you can label then handle nested loop breaking
    // label using a name and a tick mark
    'outer: loop {
        println!("loop forever");
        loop {
            println!("inner loop");
            break 'outer;
        }
    }

    // can return VALUES from a loop by using them as a break
    let x = loop {
        break 5;
    };
    println!("{x}");
    // note the semicolon at the end of the loop bracket! ^^

    // while loop
    let mut a: i32 = 0;

    while a < 5 {
        println!("a is {a}");
        a = a + 1;
        //above we are mutating a so we need to create a initally as a mutable variable

    }

        // for loop
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    for element in a {
            println!("{}", element);
    }
    // wants me to type as "for element: i32 in a {}" however this does not compile. 
    // It seems you can't type annotate a name you are using for a for loop?? 
}