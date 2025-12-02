use std::collections::HashMap;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u64 {
    let mut curpos = 50;
    let mut count = 0;

    for y in input.split_ascii_whitespace() {
        let left = y.chars().next().unwrap() == 'L';
        let amount = y[1..].parse::<i64>().unwrap();
        if left {
            curpos -= amount;
        } else {
            curpos += amount;
        }
        curpos = curpos % 100;

        if curpos == 0 {
            count += 1;
        }
    }
    count
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i64 {
    let mut curpos = 50;
    let mut count = 0;

    for y in input.split_ascii_whitespace() {
        let left = y.chars().next().unwrap() == 'L';
        let mut amount = y[1..].parse::<i64>().unwrap();
        count += amount / 100;
        amount = amount % 100;
        let lpos = curpos;
        if amount > 0 {
            if left {
                curpos -= amount;
            } else {
                curpos += amount;
            }
            if curpos < 0 {
                curpos += 100;
                if curpos != 0 && lpos != 0 {
                    count += 1;
                }
            }
            if curpos >= 100 {
                curpos -= 100;
                if curpos != 0 {
                    count += 1;
                }
            }

            if curpos == 0 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 3);
        assert_eq!(part2(TESTLIST), 6);
    }
}
