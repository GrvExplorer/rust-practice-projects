
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

    tell_longest_string();

    // let s1= String::from("20"); // heap allocated string
    // let s2 = &s1;
    // print_borrowing(&s1);

    // let s1 = String::from("40"); // heap allocated string

    // print_borrowing(&s1);
    // print_borrowing(s2);
    // println!("{} {}",s1, s2);

    // check_mut()
} 

fn print_borrowing(s: &String){
    println!("-------------");
    println!("{}",s);
    println!("-------------");
}

fn check_mut() {

    let mut x = vec![1, 2, 4];

    
    x.push(8);
    x.push(10);

    println!("{:?}", x);
    
    let last = x.last().unwrap();
    let first = x.first().unwrap();


    println!("{:?}", last);
    println!("{:?}", first);

}

fn tell_longest_string() {
    let s1 = String::from("Hello I am Using Rust.");
    let s2 = String::from("HIMUR");

    let result = longest(s1.as_str(), s2.as_str());
    println!("The longest sting is {}", result)
}

fn longest<>(a: &str, b: &str) -> &str {
    if a.len() > b.len() {
        a
    }else {
        b
    }
}