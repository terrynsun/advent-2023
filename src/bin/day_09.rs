use advent_2023::puzzle::Puzzle;
use advent_2023::util::split_parse;

fn next(vs: &Vec<i64>) -> Vec<i64> {
    vs.windows(2).map(|v| v[1] - v[0]).collect::<Vec<i64>>()
}

fn zeros(vs: &Vec<i64>) -> bool {
    vs.iter().all(|v| *v == 0)
}

fn gen_seq(vs: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut cur = vs.clone();
    let mut acc = vec![];

    while !zeros(&cur) {
        acc.push(cur.clone());
        cur = next(&cur);
    }

    acc.push(cur.clone());

    acc
}

// -- part 1 -- //
fn predict_next(vs: &Vec<i64>) -> i64 {
    gen_seq(vs).iter().map(|seq| seq[seq.len()-1]).sum::<i64>()
}

fn a(data: &Vec<Vec<i64>>) -> i64 {
    data.iter().map(predict_next).sum()
}

// -- part 2 -- //
fn predict_prev(vs: &Vec<i64>) -> i64 {
    gen_seq(vs).iter()
        .map(|seq| seq[0])
        .rev()
        .fold(0, |acc, x| x-acc)
}

fn b(data: &Vec<Vec<i64>>) -> i64 {
    data.iter().map(predict_prev).sum()
}

fn main() {
    Puzzle {
        name: "9",
        parts: vec![a, b],
        delimiter: '\n',
        preprocess: |text| {
            text.iter()
                .map(|line| split_parse(line)).collect()
        },
    }.solve();
}
