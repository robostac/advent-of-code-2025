use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> (i64, i64, HashSet<(i64, i64)>) {
    let ids = input.split_ascii_whitespace().collect::<Vec<_>>();
    let mut grid = HashSet::new();
    let mut start = 0;
    let maxy = ids.len() as i64;
    for (y, s) in ids.iter().enumerate() {
        for (x, v) in s.chars().enumerate() {
            if v == 'S' {
                start = x as i64;
            } else if v == '^' {
                grid.insert((x as i64, y as i64));
            }
        }
    }
    (start, maxy, grid)
}
#[aoc(day7, part1)]
pub fn part1(input: &str) -> i64 {
    let (start, maxy, grid) = parse(input);
    let mut current = HashSet::new();
    current.insert(start);
    let mut splits = 0;
    for y in 1..maxy {
        let mut next = HashSet::new();
        for x in current {
            if grid.contains(&(x, y)) {
                splits += 1;
                next.insert(x + 1);
                next.insert(x - 1);
            } else {
                next.insert(x);
            }
        }
        current = next;
    }
    splits
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> i64 {
    let (start, maxy, grid) = parse(input);
    let mut current = HashMap::new();
    current.insert(start, 1);

    for y in 1..maxy {
        let mut next = HashMap::new();
        for (x, count) in current {
            if grid.contains(&(x, y)) {
                for nx in [x + 1, x - 1] {
                    *next.entry(nx).or_insert(0) += count;
                }
            } else {
                *next.entry(x).or_insert(0) += count;
            }
        }
        current = next;
    }
    current.values().sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 21);
        assert_eq!(part2(TESTLIST), 40);
    }
}
