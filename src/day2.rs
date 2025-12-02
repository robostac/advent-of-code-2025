use std::collections::{HashMap, HashSet};

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u64 {
    let ids = input.split(',').collect::<Vec<_>>();
    let mut sum = 0;
    for x in ids {
        let (lower, upper) = x.split_once('-').unwrap();
        let lower = lower.parse::<u64>().unwrap();
        let upper = upper.parse::<u64>().unwrap();

        let mut value = 1;
        let mut step = 10;
        loop {
            let c = value + value * step;
            if c > upper {
                break;
            }
            if c >= lower {
                sum += c;
            }
            value += 1;
            if value == step {
                step *= 10;
            }
        }
    }
    sum
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u64 {
    let ids = input.split(',').collect::<Vec<_>>();
    let mut sum = 0;
    let mut seen = HashSet::new();
    for x in ids {
        let (lower, upper) = x.split_once('-').unwrap();
        let lower = lower.parse::<u64>().unwrap();
        let upper = upper.parse::<u64>().unwrap();

        let mut value = 1;
        let mut step: u64 = 10;
        let mut over = false;
        while over == false {
            let mut c = value;
            for i in 1..10 {
                c += value * step.pow(i);
                if c > upper {
                    if i == 1 && c > upper {
                        over = true;
                    }
                    break;
                }
                if c >= lower && seen.insert(c) {
                    sum += c;
                }
            }
            value += 1;
            if value == step {
                step *= 10;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 1227775554);
        assert_eq!(part2(TESTLIST), 4174379265);
    }
}
