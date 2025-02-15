fn main() {
    let result = factorial(5);
    println!("The factorial of 5 is: {}", result);
}

fn factorial(n: i32) -> i32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}