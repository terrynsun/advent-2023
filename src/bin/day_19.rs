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
    x: u64, // cool
    m: u64, // musical
    a: u64, // aerodynamic
    s: u64, // shiny
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

#[derive(Debug, Clone)]
struct IfClause {
    category: Category,
    operator: Comparison,
    val: u64,
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

type Path = (String, Vec<IfClause>);

fn find_accepted_paths(program: &HashMap<String, IfStmt>) -> Vec<Vec<IfClause>> {
    let mut accepted_paths = vec![];
    let mut queue: Vec<Path> = vec![("in".to_string(), vec![])];
    // Need to move from the "in" node to "A" nodes, collecting all edges
    // that are passed along the way.
    // Is this a DAG? probably not, but maybe we'll start with assuming that it is.
    loop {
        if queue.is_empty() {
            return accepted_paths;
        }

        let cur_path = queue.remove(0);
        if cur_path.0 == "A" {
            accepted_paths.push(cur_path.1);
            continue;
        }

        // drop rejected paths and continue
        if cur_path.0 == "R" {
            continue;
        }

        // push all possible outputs into the queue to check
        let node = &program[&cur_path.0];
        for possible_then_node in node.ifs.iter() {
            let mut new_clause_path = cur_path.1.clone();
            new_clause_path.push(possible_then_node.clone());

            queue.push((possible_then_node.then.clone(), new_clause_path));
        }

        // including the else path, which does not add an if-clause
        queue.push((node.terminal.clone(), cur_path.1.clone()))
    }
}

fn score(part: &Part) -> u64 {
    part.x + part.m + part.a + part.s
}

fn a(data: &(HashMap<String, IfStmt>, Vec<Part>)) -> u64 {
    let program = &data.0;
    data.1
        .iter()
        .filter(|&part| is_accepted(program, part))
        .map(score)
        .sum()
}

// for a single path, find restrictions on what passes
fn high_low_parts(path: &Vec<IfClause>) -> (Part, Part) {
    let mut low = Part {
        x: 1,
        m: 1,
        a: 1,
        s: 1,
    };

    let mut high = Part {
        x: 4000,
        m: 4000,
        a: 4000,
        s: 4000,
    };

    for edge in path {
        match edge.operator {
            // travel when part < clause ( x < 100 )
            // max acceptable value is clause-1
            Comparison::Lt => match edge.category {
                Category::X => high.x = std::cmp::min(high.x, edge.val - 1),
                Category::M => high.m = std::cmp::min(high.m, edge.val - 1),
                Category::A => high.a = std::cmp::min(high.a, edge.val - 1),
                Category::S => high.s = std::cmp::min(high.s, edge.val - 1),
            },
            // travel when part > clause
            // min acceptable value is clause+1
            Comparison::Gt => match edge.category {
                Category::X => low.x = std::cmp::max(low.x, edge.val + 1),
                Category::M => low.m = std::cmp::max(low.m, edge.val + 1),
                Category::A => low.a = std::cmp::max(low.a, edge.val + 1),
                Category::S => low.s = std::cmp::max(low.s, edge.val + 1),
            },
        }
    }

    (high, low)
}

fn count(high: &Part, low: &Part) -> u64 {
    println!("{} {} {} {}",
        high.x - low.x,
        high.m - low.m,
        high.a - low.a,
        high.s - low.s,
    );
    [
        high.x - low.x,
        high.m - low.m,
        high.a - low.a,
        high.s - low.s,
    ]
    .iter()
    .product()
}

fn b(data: &(HashMap<String, IfStmt>, Vec<Part>)) -> u64 {
    let program = &data.0;
    let accepted_paths = find_accepted_paths(program);
    let high_low_for_all_paths: Vec<_> = accepted_paths.iter().map(high_low_parts).collect();

    accepted_paths.iter().for_each(|p| {
        let (h, l) = high_low_parts(p);
        println!("{:?}", p);
        println!("h: {:?}, l: {:?}", h, l);
        println!("{}", count(&h, &l) / (4000*4000*4000*40));
    });

    high_low_for_all_paths.iter().map(|(high, low)| {
        count(high, low)
    }).sum()
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
