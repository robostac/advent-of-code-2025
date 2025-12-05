use std::collections::{HashMap, HashSet};

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    let ids = input.split_ascii_whitespace().collect::<Vec<_>>();
    let mut sum = 0;
    let mut grid = HashMap::new();
    for (y, s) in ids.iter().enumerate() {
        for (x, v) in s.chars().enumerate() {
            if v == '@' {
                grid.insert((x as i64, y as i64), v);
            }
        }
    }
    for c in grid.keys() {
        let mut count = -1;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if grid.contains_key(&(c.0 + dx, c.1 + dy)) {
                    count += 1;
                }
            }
        }
        if count < 4 {
            sum += 1;
        }
    }
    sum
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u64 {
    let ids = input.split_ascii_whitespace().collect::<Vec<_>>();
    let mut sum = 0;
    let mut grid = HashMap::new();

    let mut queue = Vec::new();
    for (y, s) in ids.iter().enumerate() {
        for (x, v) in s.chars().enumerate() {
            if v == '@' {
                grid.insert((x as i64, y as i64), -1);
            }
        }
    }
    let rolls = grid.keys().cloned().collect::<HashSet<_>>();
    for (c, v) in grid.iter_mut() {
        for dx in -1..=1 {
            for dy in -1..=1 {
                if rolls.contains(&(c.0 + dx, c.1 + dy)) {
                    *v += 1;
                }
            }
        }
        if *v < 4 {
            queue.push(*c);
        }
    }
    while let Some(c) = queue.pop() {
        if let Some(_) = grid.remove(&c) {
            sum += 1;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let np = (c.0 + dx, c.1 + dy);
                    if let Some(adj) = grid.get_mut(&np) {
                        *adj -= 1;
                        if *adj == 3 {
                            queue.push(np);
                        }
                    }
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 13);
        assert_eq!(part2(TESTLIST), 43);
    }
}
