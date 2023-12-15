use std::collections::HashMap;

use advent_2023::puzzle::Puzzle;

fn hash_inner(v: u64, c: char) -> u64 {
    let v = v + (c as u64);
    (v * 17) % 256
}

fn hash(s: &String) -> u64 {
    s.chars().fold(0, hash_inner)
}

fn a(data: &Vec<String>) -> u64 {
    data.iter()
        .map(hash)
        .sum()
}

#[derive(Default, Debug, Clone)]
struct Box(Vec<(String, u64)>);

impl Box {
    fn find(&self, s: &String) -> Option<usize> {
        self.0.iter().position(|(box_s, _)| box_s == s)
    }

    pub fn remove(&mut self, s: &String) {
        let _ = self.find(s)
            .map(|i| self.0.remove(i));
    }

    pub fn update(&mut self, s: &String, n: u64) {
        if let Some(idx) = self.find(s) {
            self.replace(idx, n);
        } else {
            self.insert(s, n);
        }
    }

    fn replace(&mut self, idx: usize, n: u64) {
        if let Some((_, old_elem)) = self.0.get_mut(idx) {
            *old_elem = n;
        }
    }

    fn insert(&mut self, s: &String, n: u64) {
        self.0.push((s.clone(), n));
    }

    fn value(&self, box_num: u8) -> u64 {
        self.0.iter()
            .enumerate()
            .map(|(i, (_, n))| (box_num as u64 + 1) * (i as u64 + 1) * n)
            .sum()
    }
}

fn b(data: &Vec<String>) -> u64 {
    let mut boxes: HashMap<u8, Box> = HashMap::new();
    (0..=255).for_each(|i| { boxes.insert(i, Default::default()); } );

    for cmd in data {
        let parts: Vec<_> = cmd.split(['-', '='])
            .map(|s| s.parse::<String>().unwrap())
            .collect();

        let label = &parts[0];
        let box_n = &(hash(label) as u8);
        if parts[1].is_empty() {
            // label-
            boxes.get_mut(box_n).unwrap().remove(label);
        } else {
            // label=n
            let n = parts[1].parse::<u64>().unwrap();
            boxes.get_mut(box_n).unwrap().update(label, n);
        }
    }

    boxes.iter()
        .map(|(&box_num, b)| b.value(box_num))
        .sum()
}

fn main() {
    Puzzle {
        name: "15",
        parts: vec![a, b],
        delimiter: ',',
        preprocess: |text| {
            text
        },
    }.solve();
}
