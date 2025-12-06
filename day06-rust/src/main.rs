fn main() {
    let part = std::env::args().skip(1).next().unwrap_or("1".to_string());
    if part == "1" {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let lines = std::io::stdin()
        .lines()
        .filter_map(|l| l.ok())
        .collect::<Vec<_>>();

    let mut lines = lines.iter().rev();
    let ops = lines.next().unwrap().split_whitespace().collect::<Vec<_>>();
    let mut results = ops.iter().map(|o| if *o == "+" { 0 } else { 1 }).collect::<Vec<_>>();
    for l in lines {
        for (i, v) in l.split_whitespace().enumerate() {
            let value = v.parse::<u64>().unwrap();
            results[i] = if ops[i] == "+" {
                results[i] + value
            } else {
                results[i] * value
            };
        }
    }

    println!("{}", results.into_iter().sum::<u64>());
}

fn part2() {
    // collect grid
    let mut grid = std::io::stdin()
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let columns = grid.iter().map(|row| row.len()).max().unwrap();
    let ops = grid[grid.len()-1].iter().filter(|c| **c != ' ').collect::<Vec<_>>();

    // transpose
    let mut transpose = Vec::new();
    for c in 0..columns {
        transpose.push(Vec::new());
        for r in 0..grid.len()-1 {
            transpose[c].push(grid[r][c]);
        }
    }

    let mut nums = transpose.iter()
        .map(|r| r.iter().fold(None, |acc, d| if *d == ' ' {
            acc
        } else {
            Some(acc.unwrap_or(0) * 10 + d.to_digit(10).unwrap() as u64)
        }));
            
    let mut result = 0;

    for op in ops {
        let mut this_result = if *op == '+' { 0 } else { 1 };
        while let Some(n) = nums.next().unwrap_or(None) {
            if *op == '+' {
                this_result += n;
            } else {
                this_result *= n;
            }
        }
        result += this_result;
    }

    println!("{}", result);
}
