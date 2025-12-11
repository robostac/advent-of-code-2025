use std::collections::{hash_map::Entry, HashMap};

type Node = std::cell::RefCell<NodeImpl>;
#[derive(Debug, Clone)]
struct NodeImpl {
    id: usize,
    children: Vec<usize>,
    incoming: usize,
    value: [i64; 4],
    key: i64,
}

impl NodeImpl {
    fn new() -> Node {
        NodeImpl::new_filled(0, 0)
    }

    fn new_filled(id: usize, value: i64) -> Node {
        std::cell::RefCell::new(NodeImpl {
            id: id,
            children: Vec::new(),
            incoming: 0,
            value: [value; 4],
            key: 0,
        })
    }

    fn add_edge(&mut self, other: usize) {
        self.children.push(other);
    }
}
#[derive(Debug, Clone)]
struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new(sz: usize, start: usize) -> Graph {
        Graph {
            nodes: (0..(sz + start))
                .map(|x| NodeImpl::new_filled(x, 0))
                .collect::<Vec<_>>(),
        }
    }

    fn add_directed_edge(&mut self, a: usize, b: usize) {
        self.nodes[a].borrow_mut().add_edge(b);
        self.nodes[b].borrow_mut().incoming += 1;
    }

    fn add_undirected_edge(&mut self, a: usize, b: usize) {
        self.add_directed_edge(a, b);
        self.add_directed_edge(b, a);
    }
}

fn parse(input: &str) -> (Graph, HashMap<String, usize>) {
    let mut hm = HashMap::new();
    let mut g = Graph::new(0, 1);

    for line in input.split('\n') {
        let (source, dests) = line.split_once(':').unwrap();
        let src_id;
        if let Some(v) = hm.get(source) {
            src_id = *v;
        } else {
            src_id = g.nodes.len();
            hm.insert(source.to_owned(), src_id);
            g.nodes.push(NodeImpl::new_filled(src_id, -1));
        }

        for d in dests.split_ascii_whitespace() {
            let dest_id;
            if let Some(v) = hm.get(d) {
                dest_id = *v;
            } else {
                dest_id = g.nodes.len();
                hm.insert(d.to_owned(), dest_id);
                g.nodes.push(NodeImpl::new_filled(dest_id, -1));
            }
            g.add_directed_edge(src_id, dest_id);
        }
    }
    (g, hm)
}

fn dfs(g: &Graph, node: usize, prev: usize, tgt: usize, key: i64) -> i64 {
    let mut count = 0;

    if node == tgt {
        if key != 3 {
            return 0;
        }
        return 1;
    }
    let key = key | g.nodes[node].borrow().key;
    if g.nodes[node].borrow().value[key as usize] >= 0 {
        return g.nodes[node].borrow().value[key as usize];
    }
    for &c in g.nodes[node].borrow().children.iter() {
        if c == prev {
            continue;
        }
        count += dfs(g, c, node, tgt, key);
    }
    g.nodes[node].borrow_mut().value[key as usize] = count;
    g.nodes[node].borrow().value[key as usize]
}
#[aoc(day11, part1)]
pub fn part1(input: &str) -> i64 {
    let (g, id_map) = parse(input);
    dfs(&g, id_map["you"], 0, id_map["out"], 3)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> i64 {
    let (g, id_map) = parse(input);
    for (i, v) in ["dac", "fft"].iter().enumerate() {
        let id = id_map[*v];
        g.nodes[id].borrow_mut().key = 1 << i;
    }

    dfs(&g, id_map["svr"], 0, id_map["out"], 0)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 5);
        // assert_eq!(part2(TESTLIST), 6);
    }

    const TEST2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
    #[test]
    fn sample2() {
        assert_eq!(part2(TEST2), 2);
        // assert_eq!(part2(TESTLIST), 6);
    }
}
