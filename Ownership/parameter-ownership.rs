// Function Parameter Ownership Transfer 

fn main() {
    let s = String::from("hello");
    take_ownership(s);
    // println!("String in main: {}", s);
}
fn take_ownership(some_string: String) {
    println!("String inside function: {}", some_string);
}