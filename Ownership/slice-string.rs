// Slicing a String 

fn main() {
    let s = String::from("hello, world");
    let hello = &s[0..5];
    let world = &s[7..12];
    println!("{}, {}", hello, world);
}