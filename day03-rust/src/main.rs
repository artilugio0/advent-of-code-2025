fn main() {
    let part = std::env::args().skip(1).next().unwrap_or("1".to_string());
    if part == "1" {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let lines = std::io::stdin().lines();

    let mut joltage = 0;
    for l in lines {
        let (mut c1, mut c2) = ('0', '0');
        for c in l.unwrap().chars() {
            if c2 > c1 {
                (c1, c2) = (c2, c);
            } else if c > c2 {
                c2 = c;
            }
        }

        joltage += (c1 as u32 - 48) * 10 + (c2 as u32 - 48);
    }

    println!("{}", joltage);
}

fn part2() {
    let lines = std::io::stdin().lines();

    let mut joltage = 0;
    for l in lines {
        let batteries = &mut ['0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0'];

        'outer: for c in l.unwrap().chars() {
            for i in 0..11 {
                if batteries[i+1] > batteries[i] {
                    for j in i..11 {
                        batteries[j] = batteries[j+1];
                    }
                    batteries[11] = c;
                    continue 'outer;
                }
            }
                
            if c > batteries[11] {
                batteries[11] = c;
            }
        }

        let bank_joltage = batteries
            .into_iter()
            .map(|b| *b as u64 - 48)
            .reduce(|acc, b| acc*10 + b)
            .unwrap_or(0);

        joltage += bank_joltage;
    }

    println!("{}", joltage);
}
