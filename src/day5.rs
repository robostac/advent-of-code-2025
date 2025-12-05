fn parse(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut fresh = Vec::new();
    let mut available = Vec::new();
    let mut blank = false;
    for x in input.split('\n') {
        if x == "" {
            blank = true;
        } else if blank == false {
            let (a, b) = x.split_once('-').unwrap();
            fresh.push((a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()));
        } else {
            available.push(x.parse::<i64>().unwrap());
        }
    }
    fresh.sort();
    (fresh, available)
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> u64 {
    let (fresh, available) = parse(input);

    available
        .iter()
        .filter(|&&y| fresh.iter().any(|(s, e)| y >= *s && y <= *e))
        .count() as u64
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> i64 {
    let (fresh, _) = parse(input);
    let mut count = 0;
    let mut cure = 0;
    for (s, e) in fresh {
        if s > cure {
            count += e - s + 1;
        } else if e > cure {
            count += e - cure;
        }
        cure = e.max(cure);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 3);
        assert_eq!(part2(TESTLIST), 14);
    }
}
