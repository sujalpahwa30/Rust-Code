// Control Flow 

// fn main() {
//     let number = 7;
//     if number < 10 {
//         println!("The number is less than 10");
//     }
// }

// fn main() {
//     let number = 15;
//     if number < 10 {
//         println!("The number is less than 10");
//     } else {
//         println!("The number is 10 or greater");
//     }
// }

// fn main() {
//     let number = 20;
//     if number < 10 {
//         println!("The number is less than 10");
//     } else if number < 20 {
//         println!("The number is between 10 and 19");
//     } else {
//         println!("The number is 20 or greater");
//     }
// }


// Combining Conditions 
// fn main() {
//     let number = 25;
//     if number % 5 == 0 && number % 2 == 0 {
//         println!("The number is divisible by both 5 and 2");
//     } else if number % 5 == 0 {
//         println!("The number is divisible by 5 but not by 2");
//     } else {
//         println!("The number is not divisible by 5");
//     }
// }

fn main() { // Using if in a let statement 
    let condition = true;
    let number = if condition { 5 } else { 10 };
    println!("The value of number is: {}", number);
}