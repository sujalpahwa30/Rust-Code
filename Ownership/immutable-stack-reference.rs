// Immutable References with Stack-Allocated Data

fn main() {
    let x = 42;
    let r1 = &x;
    let r2 = &x;
    println!("r1: {}, r2: {}", r1, r2);
}