// Pattern matching with match (match is similar to switch statement in other languages)

// fn main() {
//     let number = 3;
//     match number {
//         1 => println!("One"),
//         2 => println!("Two"),
//         3 => println!("Three"),
//         _ => println!("Something else"),
//     }
// }

// OR operator 

// fn main() {
//     let number = 2;
//     match number {
//         1 | 2 | 3 => println!("One, two or three"),
//         _ => println!("Something else"),
//     }
// }

// Matching ranges 

// fn main() {
//     let number = 7; 
//     match number {
//         1..=5 => println!("Between one and five"),
//         6..=10 => println!("Between six and ten"),
//         _ => println!("Something else"),
//     }
// }

// Destructuring with match 

// fn main() {
//     let pair = (2, -2);
//     match pair {
//         (x, y) if x == y => println!("The numbers are equal"),
//         (x, y) if x + y == 0 => println!("The numbers are opposites"),
//         _ => println!("No special properties"),
//     }
// }

// Ignoring values with patterns 

fn main() {
    let pair = (2, 8);
    match pair {
        (x, _) => println!("The first number is: {}", x),
    }
}