fn main() {
    let mut s = String::from("hello");

    {
        let slice1 = &s[0..5]; // Immutable slice
        let slice2 = &s[0..5]; // Another immutable slice
        println!("slice1: {}, slice2: {}", slice1, slice2); // Both slices are valid
    } // Immutable slices go out of scope here

    let slice3 = &mut s[0..5]; // Mutable slice
    s.replace_range(0..5, "hi");
    println!("s: {}", s); // Mutable slice modifies the original string
}