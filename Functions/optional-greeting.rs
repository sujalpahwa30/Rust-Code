fn main() {
    greet_optionally(Some("Sujal Pahwa"));
    greet_optionally(None);
}

fn greet_optionally(name: Option<&str>) {
    match name {
        Some(n) => println!("Hello, {}!", n),
        None => println!("Hello, stranger!"),
    }
}