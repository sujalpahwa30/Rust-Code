// Basic Slicing of an Array 

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    println!("{:?}", slice);
}