// Function Ownership with Clone 

fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    takes_ownership(s2);
    println!("s1 is still valid: {}", s1);
}

fn takes_ownership(some_string: String) {
    println!("String inside function: {}", some_string);
}