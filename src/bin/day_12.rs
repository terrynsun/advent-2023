use advent_2023::puzzle::Puzzle;

#[derive(Debug)]
struct RowRecord {
    springs: Vec<char>,
    counts: Vec<u32>,
}

// Lazy recursive
fn gen_all_possible_unknowns(springs: &Vec<char>) -> Vec<Vec<char>> {
    match springs.iter().position(|&x| x == '?') {
        None => {
            vec![springs.clone()]
        }
        Some(i) => {
            let mut springs = springs.clone();
            // shortcut the recursion a little by skipping non-unknown characters
            let suffixes = gen_all_possible_unknowns(&springs.split_off(i+1));

            // remove the ? and rename
            let mut prefix = springs;
            prefix.pop();

            // awful
            suffixes.clone().into_iter()
            .flat_map(|suffix| {
                vec![
                    vec![prefix.clone(), vec!['.'], suffix.clone()].into_iter().flatten().collect::<Vec<_>>(),
                    vec![prefix.clone(), vec!['#'], suffix.clone()].into_iter().flatten().collect::<Vec<_>>(),
                ]
            })
            .collect::<Vec<_>>()
        }
    }
}

fn matches(springs: &Vec<char>, counts: &Vec<u32>) -> bool {
    let mut state = '.';
    let mut cur_broken = 0;
    let mut counts_from_springs = vec![];

    for spring in springs {
        match (state, spring) {
            // broken to unbroken
            ('#', '.') => {
                counts_from_springs.push(cur_broken);
                cur_broken = 0;
            },
            // unbroken to broken
            ('.', '#') => cur_broken += 1,
            // broken to broken
            ('#', '#') => cur_broken += 1,
            // unbroken to unbroken -> ignore
            ('.', '.') => (),
            _ => panic!("invalid state while checking match"),
        }
        state = *spring;
    }

    if state == '#' {
        counts_from_springs.push(cur_broken);
    }

    counts == &counts_from_springs
}

fn a(data: &Vec<RowRecord>) -> usize {
    data.iter()
        .map(|row| {
            gen_all_possible_unknowns(&row.springs)
                .iter()
                .map(|possible_row| matches(possible_row, &row.counts))
                .filter(|&x| x)
                .count()
        })
        .sum()
}

fn b(_data: &Vec<RowRecord>) -> usize {
    0
}

fn main() {
    Puzzle {
        name: "12",
        parts: vec![a, b],
        delimiter: '\n',
        preprocess: |text| {
            text.iter().map(|line| {
                let parts = advent_2023::util::split_to_strings(line);
                let springs = parts[0].chars().collect();
                let counts = split_commas(&parts[1]);

                RowRecord { springs, counts }
            }).collect::<Vec<_>>()
        },
    }.solve();
}

pub fn split_commas(s: &str) -> Vec<u32> {
    s.split(',')
        .map(|s| s.parse::<_>().unwrap())
        .collect()
}
