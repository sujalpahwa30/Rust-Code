// Ownership and Automatic Memory Deallocation 

fn main() {
    let s = create_string();
    println!("String in main: {}", s);
}
fn create_string() -> String {
    let s = String::from("hello");
    println!("String inside function: {}", s);
    s 
}