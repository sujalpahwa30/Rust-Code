// Returning cloned data from Functions

fn main() {
    let s1 = String::from("hello");
    let s2 = clone_and_return(s1.clone());
    println!("s1: {}, s2: {}", s1, s2);
}
fn clone_and_return(s: String) -> String {
    s.clone()
}