// For loop 

// fn main() {
//     let numbers = [1,2,3,4,5];
//     for num in numbers {
//         println!("The number is: {}", num);
//     }
// }

// Iterating over a range

// fn main() {
//     for num in 1..6 {
//         println!("The number is: {}", num);
//     }
// }

// While loop

// fn main() {
//     let mut number = 3;
//     while number != 0 {
//         println!("{}!", number);
//         number -= 1;
//     }
//     println!("Liftoff!");
// }

// Combining conditions in while loops 

// fn main() {
//     let mut number = 4;
//     let limit = 10;
//     while number < limit && number % 2 == 0 {
//         println!("The number is: {}", number);
//         number += 2;
//     }
//     println!("The loop ended at: {}", number);
// }

// Using break and continue with for and while loops

// fn main() {
//     for num in 1..10 {
//         if num == 5 {
//             println!("Breaking at number: {}", num);
//             break;
//         }
//         if num % 2 == 0 {
//             continue;
//         }
//         println!("Number: {}", num);
//     }
// }

// fn main() {
//         for num in 1..10 {
//         if num == 5 {
//             println!("Breaking at number: {}", num);
//             break;
//         }
//         if num % 2 == 0 {
//             continue;
//         }
//         println!("Number: {}", num);
//     }
//     let mut num = 0;
//     while num < 10 {
//         num += 1;
//         if num == 5 {
//             println!("Breaking at number: {}", num);
//             break;
//         }
//         if num % 2 == 0 {
//              continue;
//         }
//         println!("Number: {}", num);
//     }
// }

fn main() {
    for num in 1..=6 {
        println!("The number is: {}", num);
    }
}