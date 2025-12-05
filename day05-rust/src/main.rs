fn main() {
    let part = std::env::args().skip(1).next().unwrap_or("1".to_string());
    if part == "1" {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let lines = std::io::stdin().lines().filter_map(|l| l.ok());

    let mut intervals = vec![];
    let mut intervals_loaded = false;

    let mut count = 0;

    for l in lines {
        if l.len() == 0 {
            intervals_loaded = true;
            continue;
        }

        if !intervals_loaded {
            let (start, end) = l.split_once("-").unwrap();
            intervals.push((start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()));
            continue;
        }

        let num = l.parse::<u64>().unwrap();
        for (start, end) in intervals.iter() {
            if *start <= num && num <= *end {
                count += 1;
                break;
            }
        }
    }

    println!("{}", count);
}

fn part2() {
    let lines = std::io::stdin().lines().filter_map(|l| l.ok());

    let mut intervals = vec![];
    let mut count = 0;

    for l in lines.take_while(|l| !l.is_empty()) {
        let (start, end) = l.split_once("-").unwrap();
        intervals.push((start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()));
    }

    intervals.sort();

    let mut joined_intervals = vec![];

    let (mut last_start, mut last_end) = intervals[0];
    for (start, end) in &intervals {
        if *start == last_start {
            last_end = *end;
        } else if *start <= last_end {
            if *end > last_end {
                last_end = *end;
            }
        } else {
            joined_intervals.push((last_start, last_end));
            (last_start, last_end) = (*start, *end);
        }
    }
    joined_intervals.push((last_start, last_end));

    for (start, end) in joined_intervals {
        count += end - start + 1
    }

    println!("{}", count);
}
