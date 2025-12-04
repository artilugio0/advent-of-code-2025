use std::io::Read;

fn main() {
    let part = std::env::args().skip(1).next().unwrap_or("1".to_string());
    if part == "1" {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let result = input
        .trim()
        .split(",")
        .map(|r| r.split("-")
                  .map(|n| n.parse::<u64>().unwrap())
                  .collect::<Vec<_>>())
        .map(|v| (v[0], v[1]))
        .fold(vec![], |mut acc, (start, end)| {
            for i in start..=end {
                let digits = (i as f64).log(10.0).ceil() as u32;
                let left = i / 10_u64.pow(digits/2);
                let right = i % 10_u64.pow(digits/2);

                if left == right {
                    acc.push(i)
                }
            }

            acc
        })
        .into_iter()
        .sum::<u64>();
    
    println!("{}", result);
}

fn part2() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let result = input
        .trim()
        .split(",")
        .map(|r| r.split("-")
                  .map(|n| n.parse::<u64>().unwrap())
                  .collect::<Vec<_>>())
        .map(|v| (v[0], v[1]))
        .fold(vec![], |mut acc, (start, end)| {
            for i in start..=end {
                let digits = (i as f64).log(10.0).ceil() as u32;
                for d in 1..=digits/2 {
                    if digits % d != 0 {
                        continue;
                    }

                    let divisor = 10_i32.pow(d) as u64;
                    let pattern = i % divisor;
                    let mut num = i;
                    while num > 0 && num % divisor == pattern {
                        num /= divisor;
                    }

                    if num == 0 {
                        acc.push(i);
                        break;
                    }
                }
            }

            acc
        })
        .into_iter()
        .sum::<u64>();
    
    println!("{}", result);
}
