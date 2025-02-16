// Cloning vs Moving 

fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);
    let s3 = String::from("world");
    let s4 = s3;
    // println!("s3: {}", s3);
    println!("s4: {}", s4);
}