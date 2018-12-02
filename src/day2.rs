use std::io::{Stdin, Read, Bytes, stdin};
use std::collections::HashMap;

pub fn run_1() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer);

    let mut twos = 0;
    let mut threes = 0;

    for line in buffer.lines() {
        let mut visited_chars: HashMap<char, i32> = HashMap::with_capacity(26);
        let mut has_twos = false;
        let mut has_threes = false;

        for c in line.chars() {
            if visited_chars.contains_key(&c) {
                visited_chars.insert(c, visited_chars.get(&c).unwrap() + 1);
            } else {
                visited_chars.insert(c, 1);
            }
        }

        if visited_chars.values().any(|&e| e == 2) {
            twos += 1;
        }

        if visited_chars.values().any(|&e| e == 3) {
            threes += 1;
        }
    }

    let result = twos * threes;
    println!("{}", &result);
}