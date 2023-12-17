use std::iter::zip;

use advent_2023::puzzle::Puzzle;
use advent_2023::util::split_parse;
use regex::Regex;

type Race = (u64, u64);

fn travel(time: u64, hold: u64) -> u64 {
    (time - hold) * hold
}

fn a(data: &Vec<Race>) -> u64 {
    data.iter()
        .map(|(t, d)| (0..*t).map(|x| travel(*t, x)).filter(|x| x > d).count() as u64)
        .product()
}

fn main() {
    Puzzle {
        name: "6",
        parts: vec![a],
        delimiter: '\n',
        preprocess: |text| {
            // Time:
            let times = split_parse(&text[0]["Time:".len()..]);
            // Distance:
            let distances = split_parse(&text[1]["Distance:".len()..]);

            zip(times, distances).collect::<Vec<_>>()
        },
    }
    .solve();

    Puzzle {
        name: "6",
        parts: vec![a],
        delimiter: '\n',
        preprocess: |text| {
            let re = Regex::new(r"\s*").unwrap();

            // Time:
            let times = re.replace_all(&text[0]["Time:".len()..], "").parse();
            // Distance:
            let distances = re.replace_all(&text[1]["Distance:".len()..], "").parse();

            zip(times, distances).collect::<Vec<_>>()
        },
    }
    .solve();
}
