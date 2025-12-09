fn parse(input: &str) -> Vec<Vec<i64>> {
    let ids = input.split_ascii_whitespace().collect::<Vec<_>>();
    let mut points = Vec::new();
    for y in ids.iter() {
        let t = y
            .split(',')
            .map(|t| t.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        points.push(t);
    }
    points
}
#[aoc(day9, part1)]
pub fn part1(input: &str) -> i64 {
    let points = parse(input);
    let mut biggest = 0;
    for (i, x) in points.iter().enumerate() {
        for v in points[i + 1..].iter() {
            let dx = (x[0] - v[0]).abs() + 1;
            let dy = (x[1] - v[1]).abs() + 1;
            biggest = biggest.max(dx * dy);
        }
    }
    biggest
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> i64 {
    let mut points = parse(input);
    let mut biggest = 0;
    let mut vertical = Vec::new();
    let mut horiz = Vec::new();
    points.push(points[0].clone());
    for i in points.windows(2) {
        if i[0][0] == i[1][0] {
            vertical.push((i[0][0], (i[0][1].min(i[1][1]), i[0][1].max(i[1][1]))));
        } else {
            horiz.push((i[0][1], (i[0][0].min(i[1][0]), i[0][0].max(i[1][0]))));
        }
    }
    vertical.sort();
    horiz.sort();

    for (i, x) in points.iter().enumerate() {
        for v in points[i + 1..].iter() {
            let left = x[0].min(v[0]);
            let right = x[0].max(v[0]);
            let top = x[1].min(v[1]);
            let bottom = x[1].max(v[1]);
            let dx = (x[0] - v[0]).abs() + 1;
            let dy = (x[1] - v[1]).abs() + 1;
            let mut valid = true;
            let mh = horiz.partition_point(|l| l.0 <= top);

            for hl in &horiz[mh..] {
                if hl.0 >= x[1].max(v[1]) {
                    break;
                }

                if hl.1 .0 <= left && hl.1 .1 > left {
                    valid = false;
                    break;
                }
                if hl.1 .0 < right && hl.1 .1 >= right {
                    valid = false;
                    break;
                }
            }
            if valid {
                let mv = vertical.partition_point(|l| l.0 <= left);
                for vl in &vertical[mv..] {
                    if vl.0 >= x[0].max(v[0]) {
                        break;
                    }
                    if vl.1 .0 <= top && vl.1 .1 > top {
                        valid = false;
                        break;
                    }
                    if vl.1 .0 < bottom && vl.1 .1 >= bottom {
                        valid = false;
                        break;
                    }
                }
            }
            if valid {
                biggest = biggest.max(dx * dy);
            }
        }
    }
    biggest
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 50);
        assert_eq!(part2(TESTLIST), 24);
    }
}
