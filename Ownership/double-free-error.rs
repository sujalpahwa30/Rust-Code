// Moving with strings and Avoiding double free errors 

fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1: {}", s1);
    println!("s2: {}", s2);
}