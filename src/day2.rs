use std::io::{Stdin, Read, stdin};
use std::collections::HashMap;
use core::str::Chars;

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

pub fn run_2() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer);
    let lines = buffer.lines().map(|s| s.chars()).collect::<Vec<Chars>>();
    let count = lines.len();

    'outer_loop: for i in 0..count {
        for j in (i + 1)..count {
            let f_word: &Chars = lines.get(i).unwrap();
            let s_word: &Chars = lines.get(j).unwrap();

            let (mismatch, result) = get_mismatch_chars(f_word.as_str(), s_word.as_str());
            if mismatch == 1 {
                println!("{}", result);
                break 'outer_loop;
            }
        }
    }
}

pub fn get_mismatch_chars(f_word: &str, s_word: &str) -> (u8, String) {
    let mut mismatch = 0;
    let mut result = String::new();

    for (c1, c2) in (f_word.chars()).zip(s_word.chars()) {
        if c1 != c2 {
            mismatch = mismatch + 1;
        } else {
            result.push(c1);
        }
    }
    (mismatch, result)
}