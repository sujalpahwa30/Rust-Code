fn main() {
    let descriptions = [
        describe_number(-5),
        describe_number(0),
        describe_number(5),
    ];
    for description in &descriptions {
        println!("{}", description);
    }
}
fn describe_number(number: i32) -> &'static str {
    match number {
        n if n < 0 => "negative",
        0 => "zero",
        _ => "positive",
    }
}