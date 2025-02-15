// Constants 

// fn main() {
//     const MAX_USERS: u32 = 1000;
//     println!("The maximum number of users is: {}", MAX_USERS);
// }


/**
 * 1. Constants are always immutable. We can't use the mut keyword with constants.
 * 2. Constants can be declared in any scope.
 * 3. Constants may only be set to a constant expression.
 */
// const SERVER_ADDRESS: &str = "192.168.1.1";
// fn main() {
//     println!("The server address is: {}", SERVER_ADDRESS);
// }

const SECONDS_IN_A_MINUTE: u32 = 60;
const MINUTES_IN_AN_HOUR: u32 = 60;
const HOURS_IN_A_DAY: u32 = 24;
const SECONDS_IN_A_DAY: u32 = SECONDS_IN_A_MINUTE * MINUTES_IN_AN_HOUR * HOURS_IN_A_DAY;
fn main() {
    println!("The number of seconds in a day is: {}", SECONDS_IN_A_DAY);
}


const PI: u32 = 3.141592653589793;
const MAX_SCORE: u32 = 100;
const DEFAULT_TIMEOUT: u64 = 30;