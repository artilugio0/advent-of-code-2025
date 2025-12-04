use std::io::{self};

fn main() {
    let part = std::env::args().skip(1).next().unwrap_or("1".to_string());
    if part == "1" {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let result = io::stdin().lines()
        .filter_map(|l| l.ok())
        // only keep lines with content
        .map(|l| l.trim().to_owned())
        .filter(|l| !l.is_empty())
        // parse input
        .map(|l| match l.split_at(1) {
            ("L", n) => -n.parse::<i16>().unwrap_or(0),
            (_, n) => n.parse::<i16>().unwrap_or(0),
        })
        // keep track of the dial value
        .scan(50, |dial, rotation| {
            *dial = (*dial + rotation) % 100;
            Some(*dial)
        })
        // count the number of times it hits 0
        .filter(|d| *d == 0)
        .count();

    println!("{}", result);
}

fn part2() {
    let lines = io::stdin().lines()
        .filter_map(|l| l.ok())
        .map(|l| l.trim().to_owned())
        .filter(|l| !l.is_empty());

    let mut dial = 50;
    let mut count = 0;

    for line in lines {
        let rotation = match line.split_at(1) {
            ("L", n) => -n.parse::<i16>().unwrap_or(0),
            (_, n) => n.parse::<i16>().unwrap_or(0),
        };

        let new_dial = dial + rotation;
        count += match new_dial {
            0 => 1,

            n if n < 0 => new_dial / (-100) + if dial > 0 { 1 } else { 0 },

            _ => new_dial / 100,
        };

        dial = ((new_dial % 100) + 100) % 100;
    }

    println!("{}", count);
}
