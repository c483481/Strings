#[cfg(test)]
mod test;

mod utils;

use crate::utils::compare_string::compare_string;

use crate::utils::convert_to_number::convert_to_number;


fn main() {
    println!("result convert from \"42\": {}", convert_to_number("42"));
    println!("result convert from \"abc\": {}", convert_to_number("abc"));
    println!("result compare string: {}", compare_string("same string", "same string"));
}
