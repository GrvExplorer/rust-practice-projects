// !! other words in rust !! *//
// Bound, annotated, shadowing, statements (not return just action), expression ( return ), allocation on the heap
use std::{io, string};
use rand::Rng;
use colored::Colorize;
use std::cmp::Ordering;

// *** using Macro ( ! ) ***

// !! Rules for OwnerShip !!
// ownership => each value have its owner (variable)
// one value can have only one owner at time
// when the owner goes out of scope, the variable get's dropped

// !! Rules for Borrowing/references !! (&)
// Borrowing => you can use mutable variable to change value many times before borrowing for immutable variable ( you can have one mutable references before & after at a given time ). After that use many time for immutable variable (if want to overwrite this don't use immutable variable after declaring mutable or vise versa)
// at the same time you can not borrow two mutable references
// References must always be valid ( lifespan of references variable > variable lifespan )

// !! Life Time !! 

/* 

Block comments:

you can document your program here without
multiple list of adding comments syntax to each line.

*/

fn main() {

    // simple data structure (not follow OwnerShip Rules)
    let string_str = "Other Guess";
    let r = string_str;


    let some_list = [10, 40, 20, 60];
    let vc = some_list;


    // println!("str: {} list: {:?}", string_str, some_list);

    // complex data structure ( follow OwnerShip Rules)
    let mut other_guess = String::from("Other Guess");
    other_guess.push_str(", world!");
    let r = &mut other_guess;
    // rule of borrowing is applied here.
    // let p = &mut other_guess;

    let mut some_vector = vec![10, 40, 20, 60];
    some_vector.push(40);
    let vc = some_vector;


    // println!("String: {} vector: {:?}", r, vc);


    /*
    
// guess_number();
    // concatenate();
    // which_marvel_character();

    const THREE_TIMES_EIGHT:u32 = 3*8;

    let vector = [10, 20, 30, 40, 50];
    let _vector1 = vector;

    for x in vector.iter() {
        // println!("{x}")
    };

    for num in 1..80 {
        println!("The Value is: {:#?}", num);
    };

    // println!("vector is here {:?}", vector);

    let a = 98_222;
    let b = 0xff;
    let c = 0o77;
    let d = 0b1111_0000;
    let e = b"A";
    let f: u8 = 255;

    let characters = '🦈';

    let byte = [0; 8];

    println!("{} {} {} {} {:?} {} {} {:?}", a, b, c, d, e, f, characters, byte);

    let mut spaces = String::new();
    io::stdin().read_line(&mut spaces).expect("Please Inpute Some Spaces!");

    spaces = "   ".to_string();
    let spaces = spaces.len();

    println!("Number of spaces: {}", spaces);

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

        println!("{r}");

        let spaces = "   ";
        let spaces: usize = spaces.len();
        
        println!("{spaces}")

     */

    println!(" ");

    let x =  "Hello world";
    let y = "some_list";
    // {
    //     let s = String::from("some thing to believe in!");
    //     b = &s;
    // }
    // println!("{b}");
    

    let k = longest(x, y);

    // println!("{k}");


    
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // println!("{r1} and {r2} ");
    // variable r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    // println!("{r3}");

    let mut s = String::from("Hello world");
    let s2 = "Hello world";
    let word = first_word(s2);
    
    println!("real word:{s} first word:{word}");
    s2.clear();
    println!("after clear: real word: {s} first word: {word}");

}


fn longest<'a>(a: &'a str, b: &'a str) -> &'a str { // lifetime parameter => 'a
    if a.len() > b.len() {
        a
    }else {
        b
    }
} // as you are returning a borrowed value (the owner of a & b should have greater lifespan then borrower (if rust does not make sure's this then we will refer the value that is dropped )) for this to not happen we use <'a>

fn first_word(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}

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

