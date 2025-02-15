// Data Types in Rust : Scalar and Compound

// Scalar Types: Represents a single value. There are four primary scalar types: integers, floating-point numbers, Booleans, and characters.
// Integer Literal in different formats 

// fn main() {
//     let decimal = 98_222;    // Decimal 
//     let hex = 0xff;   // Hexadecimal 
//     let octal = 0o77;  // Octal 
//     let binary = 0b1111_0000; // Binary 
//     let byte = b'A';  // Byte (u8 only)

//     println!("{}", decimal);
//     println!("{}", hex);
//     println!("{}", octal);
//     println!("{}", binary);
//     println!("{}", byte);
// }

// Floating 

// fn main() {
//     let x = 2.0;
//     let y: f32 = 3.0;
//     println!("{}", x);  
//     println!("{}", y);
// }

// Boolean

// fn main() {
//     let t = true;
//     let f: bool = false;
//     println!("{}", t);
//     println!("{}", f);
// }

// Character

// fn main() {
//     let c = 'z';
//     let z: char = 'Z';
//     let heart_eyed_cat = '😻';
//     println!("{}", c);
//     println!("{}", z);
//     println!("{}", heart_eyed_cat);
// }

// Compound Types: Represents multiple values. There are two compound types: tuples and arrays.

// Tuple

// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     // Destructuring the tuple
//     let (_x, y, _z) = tup;
//     println!("The value of y is: {}", y);

//     // Accessing the tuple element directly
//     let five_hundred = tup.0;
//     let six_point_four = tup.1;
//     let one = tup.2;
//     println!("The value of five_hundred is: {}", five_hundred);
//     println!("The value of six_point_four is: {}", six_point_four);
//     println!("The value of one is: {}", one);
// }

// Arrays

// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     let first = a[0];
//     let second = a[1];
//     println!("The value of first is: {}", first);
//     println!("The value of second is: {}", second);
// }

// Invalid Array Element access in Rust 

// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     let index = 6;
//     let element = a[index];
//     println!("The value of element is: {}", element);
// }


fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 3;
    match a.get(index) {
        Some(element) => println!("The value of element at index:{index} is: {element}"),
        None => println!("Invalid index"),
    }
}