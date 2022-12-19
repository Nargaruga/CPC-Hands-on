use std::io;

use crate::lib::plan_holiday;
mod lib;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buffer).expect("Failed to read line.");
    let tmp: Vec<u32> = buffer
        .split_whitespace()
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();

    buffer.clear();

    if tmp.len() < 2 {
        panic!("Missing input parameters.");
    }

    let n: usize = *tmp.get(0).expect("Missing input parameters.") as usize;
    let days: usize = *tmp.get(1).expect("Missing input parameters.") as usize;

    let mut cities: Vec<Vec<u32>> = Vec::with_capacity(n * days);
    for _i in 0..n {
        stdin.read_line(&mut buffer).expect("Failed to read line.");
        let city: Vec<u32> = buffer
            .split_whitespace()
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();

        cities.push(city);

        buffer.clear();
    }

    println!("{}", plan_holiday(n, days, cities));
}
