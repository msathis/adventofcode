use std::io::{Stdin, Read, Bytes, stdin};
use std::collections::HashSet;

pub fn run_1() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer);
    let mut total = 0;

    for line in buffer.lines() {
        let curr = line.parse::<i32>().unwrap();
        total += curr;
    }
    println!("{}", total);
}

pub fn run_2() {
    let mut buffer = String::new();
    let mut visited_freq = HashSet::new();

    stdin().read_to_string(&mut buffer);
    let mut total = 0;
    visited_freq.insert(0);

    loop {
        for line in buffer.lines() {
            let curr = line.parse::<i32>().unwrap();
            total += curr;

            if visited_freq.contains(&total) {
                println!("First frequency : {}", total);
                return ();
            }
            visited_freq.insert(total);
        }
    }
}