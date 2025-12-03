use std::collections::{HashMap, HashSet};

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    let ids = input.split_ascii_whitespace().collect::<Vec<_>>();
    let mut sum = 0;
    for x in ids {
        let mut best = 0;
        let mut prefix = 0;
        for c in x.chars().map(|y| y.to_digit(10).unwrap()) {
            best = best.max(prefix * 10 + c);
            if c > prefix {
                prefix = c;
            }
        }
        sum += best;
    }
    sum
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64 {
    let ids = input.split_ascii_whitespace().collect::<Vec<_>>();
    let mut sum = 0;
    for x in ids {
        let mut prefixes = vec![0; 13];
        for c in x.chars().map(|y| y.to_digit(10).unwrap() as u64) {
            for y in (1..=12).rev() {
                prefixes[y] = prefixes[y].max(prefixes[y - 1] * 10 + c);
            }
        }
        sum += prefixes[12];
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 357);
        assert_eq!(part2(TESTLIST), 3121910778619);
    }
}
