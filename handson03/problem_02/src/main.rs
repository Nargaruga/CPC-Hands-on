mod lib;
use std::io;

use crate::lib::count_reverse_flags;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buffer).expect("Failed to read line.");
    let _n: i32 = buffer.trim().parse().expect("Not an integer!");

    let mut houses = String::new();
    stdin.read_line(&mut houses).expect("Failed to read line.");

    match count_reverse_flags(&houses) {
        Err(why) => {
            panic!("Failed to count flags: {}", why);
        }
        Ok(flags) => {
            println!("{}", flags);
        }
    }
}
