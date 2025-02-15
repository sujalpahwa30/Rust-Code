fn main() {
    let num = 4;
    let result = is_even(num);
    println!("Is {} even? {}", num, result);
}
fn is_even(number: i32) -> bool {
    number % 2 == 0
}