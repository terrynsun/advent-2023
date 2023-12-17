use advent_2023::puzzle::Puzzle;
use advent_2023::twod::Map;

fn a(_data: &Map<u32>) -> u64 {
    0
}

fn b(_data: &Map<u32>) -> u64 {
    0
}

fn main() {
    Puzzle {
        name: "17",
        parts: vec![a, b],
        delimiter: '\n',
        preprocess: |text| {
            Map::new(
                text.into_iter()
                    .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
                    .collect(),
            )
        },
    }
    .solve();
}
