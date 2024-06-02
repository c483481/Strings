#[cfg(test)]
mod test;

mod utils;

use crate::utils::compare_string::{compare_string, safe_compare_string};

use crate::utils::convert_to_number::convert_to_number;

use crate::utils::check_string::is_email;


fn main() {
    println!("result convert from \"42\": {}", convert_to_number("42"));
    println!("result convert from \"abc\": {}", convert_to_number("abc"));
    println!("result compare string: {}", compare_string("same string", "same string"));
    println!("result compare string: {}", safe_compare_string("same string", "same string"));
    println!("is email admin@gmail.com: {}", is_email("admin@gmail.com"));
    println!("is email asdf@adsf.adsf: {}", is_email("asdf@adsf.adsf"));
    println!("is email asdf@univ.ac.id: {}", is_email("asdf@univ.ac.id"));
}
