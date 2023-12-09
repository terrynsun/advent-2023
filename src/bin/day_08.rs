use std::collections::HashMap;

use advent_2023::puzzle::Puzzle;
use regex::Regex;

#[derive(Clone)]
struct Directions {
    steps: Vec<char>,
    idx: usize,
}

impl Iterator for Directions {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let data = self.steps.get(self.idx).unwrap();
        self.idx = (self.idx + 1) % self.steps.len();
        Some(*data)
    }
}

#[derive(Clone)]
struct Graph {
    steps: Directions,
    map: HashMap<String, (String, String)>,

    a_nodes: Vec<String>,
}

fn len(data: &Graph, starting: &String) -> i32 {
    let mut i = 0;
    let mut cur_node = starting;

    for step in data.steps.clone() {
        let connections = &data.map[cur_node];

        cur_node = match step {
            'L' => &connections.0,
            'R' => &connections.1,
            _ => panic!("invalid direction"),
        };

        i += 1;
        if cur_node.ends_with("Z") {
            break
        }
    }

    i
}

fn a(data: &Graph) -> i32 {
    let mut i = 0;
    let mut cur_node = &"AAA".to_string();

    for step in data.steps.clone() {
        let connections = &data.map[cur_node];

        cur_node = match step {
            'L' => &connections.0,
            'R' => &connections.1,
            _ => panic!("invalid direction"),
        };

        i += 1;
        if cur_node == "ZZZ" {
            break
        }
    }

    i
}

fn b(data: &Graph) -> i32 {
    let x = data.a_nodes.iter().map(|n| len(data, n)).collect::<Vec<_>>();
    println!("{x}");
    // x.iter().fold(1, |acc, x| num::integer::lcm(acc, *x))
    // This overflowed the num crate, I used a LCM calculator online. lol.
    0
}

fn main() {
    Puzzle {
        name: "8",
        parts: vec![a, b],
        delimiter: '\n',
        preprocess: |text| {
            let steps = text[0].chars().collect::<Vec<_>>();
            let mut map = HashMap::new();
            let mut a_nodes = vec![];

            text[2..].iter().for_each(|line| {
                let nodes_re = Regex::new(r"([1-9A-Z]+) = \(([1-9A-Z]+), ([1-9A-Z]+)\)").unwrap();
                for (_, [node, l, r]) in nodes_re.captures_iter(line).map(|c| c.extract()) {
                    map.insert(node.to_string(), (l.to_string(), r.to_string()));
                    if node.ends_with('A') {
                        a_nodes.push(node.to_string());
                    }
                }
            });

            Graph {
                steps: Directions { steps, idx: 0},
                map,
                a_nodes,
            }
        },
    }.solve();
}
