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
    let grid = lines
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let movements: &[(isize, isize)] = &[
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let mut count = 0;
    let row_count = grid.len();
    let col_count = grid[0].len();

    for row in 0..row_count {
        for col in 0..col_count {
            if grid[row][col] != '@' {
                continue;
            }

            let mut adjacent = 0;

            for (dr, dc) in movements {
                let (newr, newc) = ((row as isize)+*dr, (col as isize)+*dc);
                if newr < 0 || newc < 0 || newr >= (row_count as isize) || newc >= (col_count as isize) {
                    continue;
                }

                if grid[newr as usize][newc as usize] == '@' {
                    adjacent += 1;
                }
            }

            if adjacent < 4 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

fn part2() {
    let lines = std::io::stdin().lines().filter_map(|l| l.ok());
    let mut grid = lines
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let movements: &[(isize, isize)] = &[
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let mut count = 0;
    let mut count_prev = -1;

    let row_count = grid.len();
    let col_count = grid[0].len();

    while count != count_prev {
        count_prev = count;

        for row in 0..row_count {
            for col in 0..col_count {
                if grid[row][col] != '@' {
                    continue;
                }

                let mut adjacent = 0;

                for (dr, dc) in movements {
                    let (newr, newc) = ((row as isize)+*dr, (col as isize)+*dc);
                    if newr < 0 || newc < 0 || newr >= (row_count as isize) || newc >= (col_count as isize) {
                        continue;
                    }

                    if grid[newr as usize][newc as usize] == '@' {
                        adjacent += 1;
                    }
                }

                if adjacent < 4 {
                    grid[row][col] = '.';
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
