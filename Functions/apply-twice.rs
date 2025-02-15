fn main() {
    let result1 = apply_twice(double, 2);
    println!("Double twice of 2 is: {}", result1);
    let result2 = apply_twice(increment, 2);
    println!("Increment twice of 2 is: {}", result2);
}

fn apply_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn double(x: i32) -> i32 {
    x * 2
}

fn increment(x: i32) -> i32 {
    x + 1
}