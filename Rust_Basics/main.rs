// # Variables and Mutability


// fn main() {
//     let y = 10;
//     println!("The value of y is: {}", y);
//     y = 20;   // By default variables are immutable in Rust
//     println!("The value of y is: {}", y);
// }


fn main() {
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 20;
    println!("The value of y is: {}", y);
}