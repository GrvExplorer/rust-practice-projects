use std::io;
use rand::Rng;

// *** using Macro ( ! ) ***

// !! Rules for OwnerShip !!
// ownership => each variable have its owner
// one variable can have only one owner at time
// when the owner goes out of scope, the variable get's dropped

// !! Rules for Borrowing/references !! (&)
// Borrowing => you can use mutable variable to change value many times before borrowing for immutable variable. After that use many time for immutable variable
// References must always be valid ( lifespan of references > variable lifespan )

// !! Life Time !! 

fn main() {
    guess_number();
}

// fn longest<'a, >(a: &'a str, b: &'a str) -> &'a str {
//     if a.len() > b.len() {
//         a
//     }else {
//         b
//     }
// }

fn guess_number() {
    println!("Guess the number");

    println!("Input your number: ");
        
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("failed to read read_line");
    
    println!("Your guessed {}", guess);
}
