// Mutable References and Modification 

fn main() {
    let mut s = String::from("hello");
    let r = &mut s;

    r.push_str(", world");
    println!("s: {}", s);
}