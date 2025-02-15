// Shadowing 

// fn main() {
//     let a = 5;
//     let a = a+1;
//     println!("The value of a is: {}", a);

//     {
//         let a = a*2;
//         println!("The value of a in the inner scope is: {}", a);
//     }

//     println!("The value of a is: {}", a);
// }

// fn main() {
//     let spaces = "   ";
//     let spaces = spaces.len();
//     println!("The value of spaces is: {}", spaces);
// }

// Shadowing vs Mutability 

// fn main() {
//     // Using mutability 
//     let mut count = 1;
//     count = 2;
//     println!("Count using mutability: {}", count);

//     // Using shadowing 
//     let count = "one";
//     let count = count.len();
//     println!("Count using shadowing: {}", count);
// }

// Scope and Shadowing 

// fn main() {
//     let number = 10;
//     {
//         let number = 20;
//         println!("Number in inner scope: {}", number);
//     }
//     println!("Number in outer scope: {}", number);
// }

// Practical Use Case 

use std::io;
fn main() {
    println!("Please enter your age: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim(); // Shadowing to remove whitespace
    let input: u32 = input.parse().expect("Please enter a number!"); // Shadowing to convert type

    println!("Your age is: {}", input);
}