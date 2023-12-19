use std::collections::HashMap;

use regex::Regex;

use advent_2023::puzzle::Puzzle;

#[derive(Debug, Copy, Clone)]
enum Category {
    X, // cool
    M, // musical
    A, // aerodynamic
    S, // shiny
}

impl From<char> for Category {
    fn from(value: char) -> Self {
        match value {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!("invalid category"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Part {
    x: u32, // cool
    m: u32, // musical
    a: u32, // aerodynamic
    s: u32, // shiny
}

#[derive(Debug, Copy, Clone)]
enum Comparison {
    Lt,
    Gt,
}

impl From<char> for Comparison {
    fn from(value: char) -> Self {
        match value {
            '>' => Self::Gt,
            '<' => Self::Lt,
            _ => panic!("invalid <> operator"),
        }
    }
}

#[derive(Debug)]
struct IfClause {
    category: Category,
    operator: Comparison,
    val: u32,
    then: String,
}

impl IfClause {
    // does the part part pass the 'if' and land in the 'then'
    fn pass(&self, input: &Part) -> bool {
        match self.operator {
            Comparison::Lt => match self.category {
                Category::X => input.x < self.val,
                Category::M => input.m < self.val,
                Category::A => input.a < self.val,
                Category::S => input.s < self.val,
            },
            Comparison::Gt => match self.category {
                Category::X => input.x > self.val,
                Category::M => input.m > self.val,
                Category::A => input.a > self.val,
                Category::S => input.s > self.val,
            },
        }
    }
}

#[derive(Debug)]
struct IfStmt {
    name: String,
    ifs: Vec<IfClause>,
    terminal: String,
}

impl IfStmt {
    fn run(&self, input: &Part) -> String {
        for stmt in self.ifs.iter() {
            if stmt.pass(input) {
                return stmt.then.clone();
            }
        }

        self.terminal.clone()
    }

    fn get_outputs(&self) -> Vec<String> {
        let mut v: Vec<_> = self.ifs.iter().map(|i| i.then.to_string()).collect();
        v.push(self.terminal.to_string());
        v
    }
}

impl From<&String> for IfStmt {
    fn from(value: &String) -> Self {
        let re = Regex::new(r"([a-z]+)\{(.*+),([a-zRA]+)\}").unwrap();
        let (_full, [name, ifs_str, terminal]) = re.captures(value).unwrap().extract();

        let if_re = Regex::new(r"([xmas])([<>])([0-9]+):([a-zRA]+)").unwrap();

        let ifs = if_re
            .captures_iter(ifs_str)
            .map(|c| c.extract())
            .map(|(_, [category, operator, val, then])| IfClause {
                category: category.chars().next().unwrap().into(),
                operator: operator.chars().next().unwrap().into(),
                val: val.parse().unwrap(),
                then: then.to_string(),
            })
            .collect();
        Self {
            name: name.to_string(),
            ifs,
            terminal: terminal.to_string(),
        }
    }
}

fn is_accepted(program: &HashMap<String, IfStmt>, part: &Part) -> bool {
    let mut state = &program["in"];
    loop {
        let next_state = state.run(part);

        if next_state == "R" {
            return false;
        } else if next_state == "A" {
            return true;
        }

        state = &program[&next_state];
    }
}

fn find_accepted_paths(program: &HashMap<String, IfStmt>) -> Vec<Vec<&IfStmt>> {
    let start = &program["in"];

    let mut accepted_paths = vec![];
    let mut queue = vec![vec![start]];
    loop {
        let cur = queue.remove(0);
        let next_node_names = cur[cur.len()-1].get_outputs();

        for next_node_name in next_node_names {
            // Drop reject paths
            if next_node_name == "R" {
                continue;
            } else if next_node_name == "A" {
                accepted_paths.push(cur.clone());
                continue;
            }

            let next_node = &program[&next_node_name];
            let mut path = cur.clone();
            path.push(next_node);

            queue.push(path);
        }

        if queue.is_empty() {
            return accepted_paths;
        }
    }
}

// fn high_low_parts(path: Vec<IfStmt>) -> (Part, Part) {
//     let low = Part {
//         x: u32::MAX,
//         m: u32::MAX,
//         a: u32::MAX,
//         s: u32::MAX,
//     };

//     let high = Part {
//         x: 0,
//         m: 0,
//         a: 0,
//         s: 0,
//     };


// }

fn score(part: &Part) -> u32 {
    part.x + part.m + part.a + part.s
}

fn a(data: &(HashMap<String, IfStmt>, Vec<Part>)) -> u32 {
    let program = &data.0;
    data.1
        .iter()
        .filter(|&part| is_accepted(program, part))
        .map(score)
        .sum()
}

fn b(data: &(HashMap<String, IfStmt>, Vec<Part>)) -> u32 {
    let program = &data.0;
    let accepted_paths = find_accepted_paths(program);
    println!("{:?}", accepted_paths.len());

    0
}

fn main() {
    Puzzle {
        name: "19",
        parts: vec![a, b],
        delimiter: '\n',
        preprocess: |mut text| {
            // split parts_list at the first non-empty line
            let idx = text.iter().position(|x| x.is_empty()).unwrap();
            let parts_list = text.split_off(idx + 1);

            // drop newline
            text.pop();

            let mut programs = HashMap::new();
            text.iter().for_each(|line| {
                let i: IfStmt = line.into();
                programs.insert(i.name.clone(), i);
            });

            let parts = parts_list
                .iter()
                .map(|line| {
                    let re =
                        Regex::new(r"\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)\}").unwrap();
                    let captures = re.captures(line).unwrap();
                    Part {
                        x: captures[1].parse().unwrap(),
                        m: captures[2].parse().unwrap(),
                        a: captures[3].parse().unwrap(),
                        s: captures[4].parse().unwrap(),
                    }
                })
                .collect();

            (programs, parts)
        },
    }
    .solve();
}
