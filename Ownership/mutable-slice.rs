fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    let slice = &mut arr[1..4]; // Mutable slice from index 1 to 4 (exclusive)
    slice[0] = 10; // Modify the first element of the slice
    println!("{:?}", arr); // Output: [1, 10, 3, 4, 5]
}