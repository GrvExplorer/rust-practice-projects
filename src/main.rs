// !! other words in rust !! *//
// Bound, annotated, shadowing, statements (not return just action), expression ( return )
use std::io;
use rand::{Error, Rng};
use colored::Colorize;
use std::cmp::Ordering;

// *** using Macro ( ! ) ***

// !! Rules for OwnerShip !!
// ownership => each variable have its owner
// one variable can have only one owner at time
// when the owner goes out of scope, the variable get's dropped

// !! Rules for Borrowing/references !! (&)
// Borrowing => you can use mutable variable to change value many times before borrowing for immutable variable. After that use many time for immutable variable
// References must always be valid ( lifespan of references variable > variable lifespan )

// !! Life Time !! 

fn main() {
    // guess_number();
    // concatenate();
    // which_marvel_character();

    const THREE_TIMES_EIGHT:u32 = 3*8;

    let a = 98_222;
    let b = 0xff;
    let c = 0o77;
    let d = 0b1111_0000;
    let e = b"A";
    let f: u8 = 255;

    let characters = '🦈';

    let byte = [0; 8];

    println!("{} {} {} {} {:?} {} {} {:?}", a, b, c, d, e, f, characters, byte);

    // let mut spaces = String::new();
    // io::stdin().read_line(&mut spaces).expect("Please Inpute Some Spaces!");

    // spaces = "   ".to_string();
    // let spaces = spaces.len();

    // println!("Number of spaces: {}", spaces);

    // i = signed, u = unsigned
    // i8 (8-bit) -(2^n-1) to 2^n-1 -1 n = number of bits (-128 to 127);
    // u8 (8-bit) 0 to 2^n -1 (0 to 255);
        let x = 5; // 1.1.1
    
        let mut x = x + 1; // 1.1.2
        {
            x = x * 2;
            println!("The value of x in the inner scope is: {x}");
        }
    
        println!("The value of x is: {x}");

        let r; // 1.1.1
        {
            let s = String::from("Let's Get Rusty!"); // 1.1.2
            r = s; // 1.1.1 have the value "Let's Get Rusty!"
        }

        println!("{r}")
        // let mut spaces = "   ";
        // spaces = spaces.len();
        
        // println!("{spaces}")

}


// fn longest<'a, >(a: &'a str, b: &'a str) -> &'a str {
//     if a.len() > b.len() {
//         a
//     }else {a
//         b
//     }
// }

#[warn(dead_code)]
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
            Ordering::Less => println!("{}", "Too Small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You guessed it!".green());
                break;
            }
        }
        
    }

    println!("secret number was {}", secret_number);
    
    return 
}

#[warn(dead_code)]
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

#[warn(dead_code)]
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

