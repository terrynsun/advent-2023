use advent_2023::puzzle::*;

use regex::Regex;

fn one_a(data: &Vec<String>) -> i32 {
    let re = Regex::new(r"([0-9])").unwrap();

    let mut sum = 0;

    for line in data {
        let mut nums = vec![];
        for (_, [n]) in re.captures_iter(line).map(|c| c.extract()) {
            nums.push(n.parse::<i32>().unwrap_or(0));
        }
        sum += (nums[0] * 10) + nums[nums.len() - 1];
    }

    sum
}

fn to_num(s: &str) -> i32 {
    if let Ok(i) = s.parse::<i32>() {
        i
    } else {
        match s {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => 0,
        }
    }
}

fn one_b(data: &Vec<String>) -> i32 {
    let first_re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|[0-9]).*").unwrap();
    let last_re = Regex::new(r".*(one|two|three|four|five|six|seven|eight|nine|[0-9]).*").unwrap();

    let mut sum = 0;

    for line in data {
        let f = &first_re.captures(line).unwrap()[1];
        let l = &last_re.captures(line).unwrap()[1];
        sum += to_num(f) * 10 + to_num(l);
    }

    sum
}

fn main() {
    Puzzle::new(
        "1",
        vec![one_a, one_b],
        '\n',
        |v| v,
    ).solve();
}
