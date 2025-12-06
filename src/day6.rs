#[aoc(day6, part1)]
pub fn part1(input: &str) -> i64 {
    let mut numbers = Vec::new();
    let mut ops = Vec::new();
    for x in input.split_ascii_whitespace() {
        if let Ok(y) = x.parse::<i64>() {
            numbers.push(y);
        } else {
            ops.push(x.chars().next().unwrap());
        }
    }
    let mut total = 0;

    for (i, c) in ops.iter().enumerate() {
        if *c == '+' {
            total += numbers.iter().skip(i).step_by(ops.len()).sum::<i64>();
        } else {
            total += numbers.iter().skip(i).step_by(ops.len()).product::<i64>();
        }
    }
    total
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> i64 {
    let mut lines = input
        .split('\n')
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut total = 0;
    let mut values = Vec::new();
    if lines.last().unwrap().len() == 0 {
        lines.pop(); //trailing newline
    }
    while lines[0].len() > 0 {
        let op = lines.last_mut().unwrap().pop().unwrap();
        let mut value = 0;
        for i in 0..(lines.len() - 1) {
            if let Some(y) = lines[i].pop() {
                if let Some(yy) = y.to_digit(10) {
                    value *= 10;
                    value += yy as i64;
                }
            }
        }
        if value == 0 {
            values.clear();
        } else {
            values.push(value);
            if op == '+' {
                total += values.iter().sum::<i64>();
            } else if op == '*' {
                total += values.iter().product::<i64>();
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 4277556);
        assert_eq!(part2(TESTLIST), 3263827);
    }
}
