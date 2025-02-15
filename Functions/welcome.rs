fn main() {
    let message = welcome_message("Sujal Pahwa");
    println!("{}", message);
}
fn welcome_message(name: &str) -> String {
    format!("Welcome, {}!", name)
}