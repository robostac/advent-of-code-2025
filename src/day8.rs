use std::collections::{HashMap, HashSet};

#[derive(Clone, Default, Debug)]
struct DSU {
    size: Vec<usize>,
    parents: Vec<usize>,
    nc: usize,
}

impl DSU {
    fn new(sz: usize, start: usize) -> DSU {
        DSU {
            size: vec![1; sz + start],
            parents: (0..(sz + start)).collect(),
            nc: sz,
        }
    }

    fn parent(&mut self, idx: usize) -> usize {
        // println!("{} {}", idx, self.parents[idx]);
        if self.parents[idx] != idx {
            let pa = self.parent(self.parents[idx]);
            self.parents[idx] = pa;
        }
        return self.parents[idx];
    }

    fn merge(&mut self, idx1: usize, idx2: usize) {
        let mut pa = self.parent(idx1);
        let mut pb = self.parent(idx2);

        if pa == pb {
            return;
        }

        if self.size[pb] > self.size[pa] {
            std::mem::swap(&mut pa, &mut pb);
        }
        self.nc -= 1;
        self.parents[pb] = pa;
        self.size[pa] += self.size[pb];
    }

    fn count(&self, idx: usize) -> usize {
        return self.size[idx];
    }
}

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
#[aoc(day8, part1)]
pub fn part1(input: &str) -> i64 {
    let points = parse(input);
    let mut count = 1000;
    if points.len() < 100 {
        count = 10; //for test
    }
    let mut dists = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let mut dd = 0;
            for z in 0..3 {
                let t = points[i][z] - points[j][z];
                dd += t * t;
            }
            dists.push((dd, i, j));
        }
    }
    dists.sort();
    let mut dsu = DSU::new(points.len(), 0);
    for (_, i, j) in &dists[..count] {
        dsu.merge(*i, *j);
    }
    let mut sections = Vec::new();
    for i in 0..points.len() {
        if dsu.parent(i) == i {
            sections.push((dsu.count(i) as i64, i));
        }
    }
    sections.sort();
    let mut answer = 1;
    for z in 1..=3 {
        answer *= sections[sections.len() - z].0;
    }
    answer
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> i64 {
    let points = parse(input);
    let mut dists = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let mut dd = 0;
            for z in 0..3 {
                let t = points[i][z] - points[j][z];
                dd += t * t;
            }
            dists.push((dd, i, j));
        }
    }
    dists.sort();
    let mut dsu = DSU::new(points.len(), 0);
    let mut li = 0;
    let mut lj = 0;
    for (_, i, j) in &dists {
        dsu.merge(*i, *j);
        let pp = dsu.parent(*i);
        if dsu.count(pp) == points.len() {
            li = *i;
            lj = *j;
            break;
        }
    }
    points[li][0] * points[lj][0]
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 40);
        assert_eq!(part2(TESTLIST), 25272);
    }
}
