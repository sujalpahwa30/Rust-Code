fn main() {
    let product = calculate_product(4, 5);
    println!("The product is: {}", product);
}
fn calculate_product(a: i32, b: i32) -> i32 {
    a * b
}