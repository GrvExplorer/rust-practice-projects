use core::num;
use std::io;
use rand::Rng;
use colored::Colorize;
use std::cmp::Ordering;

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
    // concatenate();
    // which_marvel_character();
}


// fn longest<'a, >(a: &'a str, b: &'a str) -> &'a str {
//     if a.len() > b.len() {
//         a
//     }else {a
//         b
//     }
// }

fn guess_number() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..40);
    
    loop {
        let mut guess = String::new();
        
        println!("Input your number: ");
        io::stdin().read_line(&mut guess).expect("failed to read read_line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // let guess: u32 = guess.trim().parse::<u32>().expect("Please enter a number");
        // println!("Your guessed {}", guess);


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
        }
        
    }

    println!("secret number was {}", secret_number)
        

}


fn concatenate() {

    println!("Welcome to concatenating your words !");

    
    let mut name = String::new();
    println!("What is your name? ");
    io::stdin().read_line(&mut name).expect("failed to read read_line");

    let mut age = String::new();
    println!("What is your age? ");
    io::stdin().read_line(&mut age).expect("failed to read read_line");

    println!("your name is {} and your age is {}", name.purple(), age.green());

}

fn which_marvel_character() {
    let mut heading = String::from("Which Marvel Character creator you are play");
    println!("{}", heading.green());
    println!("-----");
    println!("Use yes or no as your answer");
    println!("");
    let mut answer = String::new();

    println!("Do you like hanging around ? ");
    io::stdin().read_line(&mut answer).expect("failed to read read_line");

    if answer.to_lowercase().trim() == "yes" {
        println!("Then you are Spider-Man !")
    }else {
        println!("Do you have a gravelly voice ? ");
        io::stdin().read_line(&mut answer).expect("failed to read read_line");
        if answer.to_lowercase().trim() == "yes" {
            println!("Aww, then you're korg !")
        }else {
            println!("Do you often feel 'Marvelous' ?");
            io::stdin().read_line(&mut answer).expect("failed to read read_line");
            if answer.to_lowercase().trim() == "yes" {
                println!("You are a captain marvel !");
            }else {
                println!("Stop watching Marvel movies from write now OK!\nBetter to Coding");
            }
        }
    }

    println!("Thank you for playing !");

}

