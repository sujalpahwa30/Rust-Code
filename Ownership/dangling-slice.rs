fn main() {
    let slice;
    {
        let arr = [1, 2, 3, 4, 5];
        slice = &arr[1..4]; // Error: arr does not live long enough
    }
    // Uncommenting the next line will cause a compile-time error
    // println!("{:?}", slice);
}