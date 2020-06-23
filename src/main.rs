use std::error::Error;
use std::fs::read_to_string;

fn main() {
    match read_to_string("hello.txt") {
        Err(err) => panic!("Couldn't read: {}", err.description()),
        Ok(data) => println!("Content is: {}", data)
    }
}
