fn main() {
    let mut s = String::from("hello");
    {
        let r1 = &s;
        let r2 = &s;
        println!("r1: {}, r2: {}", r1, r2);
    }
    let r3 = &mut s;
    r3.push_str(", world");
    println!("r3: {}", r3);
}