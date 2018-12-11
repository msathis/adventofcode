use std::io::{Stdin, Read, stdin};
use std::collections::HashMap;
use core::str::Chars;
use regex::Regex;

#[derive(Debug)]
pub struct Claim {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}


pub fn parse(s: &str, re: &Regex) -> Claim {
    let matches = re.captures(s).unwrap();
    Claim {
        id: matches.get(1).unwrap().as_str().parse::<u32>().unwrap(),
        x: matches.get(2).unwrap().as_str().parse::<u32>().unwrap(),
        y: matches.get(3).unwrap().as_str().parse::<u32>().unwrap(),
        w: matches.get(4).unwrap().as_str().parse::<u32>().unwrap(),
        h: matches.get(5).unwrap().as_str().parse::<u32>().unwrap(),
    }
}

pub fn get_map(buffer: &String, re: &Regex) -> (HashMap<(u32, u32), i32>, Vec<Claim>) {
    let mut hashmap = HashMap::new();
    let mut claims = vec![];

    for line in buffer.lines() {
        let claim = parse(line, &re);
        let mut interesting = true;

        for i in claim.x..claim.x + claim.w {
            for j in claim.y..claim.y + claim.h {
                let mut count = 1;
                if hashmap.contains_key(&(i, j)) {
                    count = hashmap.get(&(i, j)).unwrap() + 1;
                    interesting = false;
                }
                hashmap.insert((i, j), count);
            }
        }
        if interesting {
            claims.push(claim);
        }
    }
    (hashmap, claims)
}

pub fn run_1() {
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer);

    let (hashmap, claims) = get_map(&buffer, &re);

    let mut count = 0;
    for (key, val) in &hashmap {
        if val > &1 {
            count += 1;
        }
    }
    println!("{}", count);
}

pub fn run_2() {
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer);

    let (hashmap, claims) = get_map(&buffer, &re);

    for claim in claims {
        let mut overlap = false;
        'outer: for i in claim.x..claim.x + claim.w {
            for j in claim.y..claim.y + claim.h {
                if hashmap.get(&(i, j)).unwrap() > &1 {
                    overlap = true;
                    break 'outer;
                }
            }
        }
        if !overlap {
            println!("{}", claim.id);
        }
    }
}

