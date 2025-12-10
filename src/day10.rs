use std::{
    collections::{HashMap, HashSet},
    i64,
};

fn parse(input: &str) -> Vec<(u64, Vec<u64>, Vec<i64>)> {
    let ids = input.split('\n').collect::<Vec<_>>();
    let mut ans = Vec::new();
    for y in ids.iter() {
        let mut buttons = Vec::new();
        let mut target = 0;
        let mut jolts = Vec::new();
        let p = y.split_ascii_whitespace().collect::<Vec<_>>();

        for (idx, c) in p[0].chars().skip(1).enumerate() {
            if c == '#' {
                target |= 1 << idx;
            }
        }

        for v in &p[1..p.len() - 1] {
            buttons.push(
                v[1..v.len() - 1]
                    .split(',')
                    .map(|c| 1 << c.parse::<u64>().unwrap())
                    .sum(),
            )
        }

        let js = p.last().unwrap();

        jolts = js[1..js.len() - 1]
            .split(',')
            .map(|c| c.parse::<i64>().unwrap())
            .collect();
        ans.push((target, buttons, jolts))
    }
    ans
}
#[aoc(day10, part1)]
pub fn part1(input: &str) -> i64 {
    let machines = parse(input);
    let mut total = 0;
    for (target, buttons, _) in machines {
        let mut queue = HashMap::new();
        queue.insert(0, 0);
        for b in buttons.iter() {
            for p in queue.clone() {
                let nc = p.1 + 1;
                let e = queue.entry(b ^ p.0).or_insert(nc);
                *e = (*e).min(nc);
            }
        }
        total += queue.get(&target).unwrap();
    }
    total
}

//ok, I'm too lazy to implement the solver for this myself
use microlp::*;
#[aoc(day10, part2)]
pub fn part2(input: &str) -> i64 {
    let machines = parse(input);
    let mut total = 0;

    for (_, buttons, jolts) in machines {
        let mut problem = Problem::new(OptimizationDirection::Minimize);
        let mut vars = Vec::new();
        for i in 0..buttons.len() {
            let mut max_presses = i32::MAX;
            for j in 0..jolts.len() {
                if buttons[i] & (1 << j) > 0 {
                    max_presses = max_presses.min(jolts[j] as i32);
                }
            }
            vars.push(problem.add_integer_var(1.0, (0, max_presses)));
        }

        for j in 0..jolts.len() {
            problem.add_constraint(
                vars.iter().enumerate().map(|(i, y)| {
                    if buttons[i] & (1 << j) > 0 {
                        (*y, 1.0)
                    } else {
                        (*y, 0.0)
                    }
                }),
                ComparisonOp::Eq,
                jolts[j] as f64,
            );
        }
        if let Ok(sol) = problem.solve() {
            total += sol.objective().round() as i64;
        } else {
            panic!();
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 7);
        assert_eq!(part2(TESTLIST), 33);
    }
}
