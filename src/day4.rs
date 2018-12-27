use std::io::{Stdin, Read, stdin};
use std::collections::HashMap;
use core::str::Chars;
use regex::Regex;
use std::cmp;

#[derive(Ord, PartialEq, PartialOrd, Eq, Debug)]
enum Action {
    BEGIN(i32),
    SLEEP,
    WAKEUP,
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug)]
struct GuardLog {
    year: i32,
    month: i32,
    date: i32,
    hour: i32,
    minute: i32,
    action: Action,
}

pub fn run_1() {
    let line_re = Regex::new(r"(?m)^\[(\d+)\-(\d+)\-(\d+) (\d+):(\d+)\] (.+)$").unwrap();
    let guard_re = Regex::new(r"^(.+)#(\d+)(.+)").unwrap();
    let mut logs = vec![];

    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer);

    for line in buffer.lines() {
        let log_matches = line_re.captures(line).unwrap();

        let action = match log_matches.get(6).unwrap().as_str() {
            "wakes up" => Action::WAKEUP,
            "falls asleep" => Action::SLEEP,
            str => {
                let captures = guard_re.captures(str).unwrap();
                Action::BEGIN(captures.get(2).unwrap().as_str().parse::<i32>().unwrap())
            }
        };
        logs.push(GuardLog {
            year: log_matches.get(1).unwrap().as_str().parse().unwrap(),
            month: log_matches.get(2).unwrap().as_str().parse().unwrap(),
            date: log_matches.get(3).unwrap().as_str().parse().unwrap(),
            hour: log_matches.get(4).unwrap().as_str().parse().unwrap(),
            minute: log_matches.get(5).unwrap().as_str().parse().unwrap(),
            action,
        });
    }
    logs.sort();
    get_max_guard(&logs);
}

fn get_max_guard(logs: &Vec<GuardLog>) {
    let mut matrix_map = HashMap::new();
    let mut total_map = HashMap::new();
    let mut prev_guard = -1;
    let mut guard_sleep_start = 60;

    for log in logs {
        match log.action {
            Action::BEGIN(i) => prev_guard = i,
            Action::SLEEP => guard_sleep_start = log.minute,
            Action::WAKEUP => {
                if prev_guard != -1 {
                    for i in guard_sleep_start..log.minute {
                        if matrix_map.contains_key(&(prev_guard, i)) {
                            matrix_map.insert((prev_guard, i), matrix_map.get(&(prev_guard, i)).unwrap() + 1);
                        } else {
                            matrix_map.insert((prev_guard, i), 1);
                        }
                    }
                    if total_map.contains_key(&prev_guard) {
                        total_map.insert(prev_guard, total_map.get(&prev_guard).unwrap() + (log.minute - guard_sleep_start));
                    } else {
                        total_map.insert(prev_guard, log.minute - guard_sleep_start);
                    }
                }
            }
        }
    }

    let mut max_guard = -1;
    let mut max_value = -1;
    total_map.iter().for_each(|(i, v)| {
        max_value = cmp::max(max_value, *v);
        if max_value == *v {
            max_guard = *i;
        }
    });

    let mut max_mins = -1;
    let mut max_mins_val = -1;
    matrix_map.iter().filter(|&(&(g, v), _)| g == max_guard).for_each(|(&(g, v), val)| {
        if max_mins_val < *val {
            max_mins_val = *val;
            max_mins = v;
        }
    });

    println!("{}", max_guard * max_mins);
}