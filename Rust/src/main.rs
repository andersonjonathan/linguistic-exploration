mod code;
mod test;

use code::*;
use std::io;

pub fn main() {

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("{}", is_palindrome(input));
        }
        Err(error) => println!("error: {}", error),
    }
}

