// Basic Loop 

// fn main() {
//     loop{
//         println!("This will run forever!");
//     }
// }

// fn main() {
//     let mut count = 0;

//     loop {
//         count += 1;
//         println!("Count: {}", count);

//         if count == 5 {
//             break;
//         }
//     }
//     println!("Loop ended at count: {}", count);
// }

// fn main() {
//     let mut count = 0;
//     let result = loop {
//         count += 1;

//         if count == 10 {
//             break count * 2;
//         }
//     };
//     println!("The result is: {}", result);
// }


// Loop Labels 
// fn main() {
//     let mut outer_count = 0;
//     'outer: loop {
//         println!("Outer count: {}", outer_count);
//         let mut inner_count = 0;

//         loop {
//             println!("Inner count: {}", inner_count);
//             inner_count += 1;

//             if inner_count == 2 {
//                 break;
//             }
//             if outer_count == 3 {
//                 break 'outer;
//             }
//         }
//         println!("----------------");
//         outer_count += 1;
//     }
//     println!("Outer loop ended at count: {}", outer_count);
// }

fn main() {
    for i in 1..=5 {
        if i == 3 {
            continue;
        } 
        println!("The number is: {}", i);
    }
}