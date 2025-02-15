// Macros 

// macro_rules! greet {
//     () => {
//         println!("Hello, Rustaceans!");
//     };
// }
// fn main() {
//     greet!();
// }


// macro_rules! greet {
//     ($name: expr) => {
//         println!("Hello, {}!", $name);
//     };
// }
// fn main() {
//     greet!("Sujal Pahwa");
//     greet!("Rustaceans");
// }


// Practical Use Cases 
// 1. Logging Macros 
// 2. println! Macro 
macro_rules! log {
    ($msg: expr) => {
        println!("[LOG]: {}", $msg);
    };
}
fn main() {
    log!("Application started");
}