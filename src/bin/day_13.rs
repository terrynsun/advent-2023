use advent_2023::puzzle::Puzzle;

#[derive(Debug)]
struct Pattern(Vec<Vec<char>>);

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self.0.iter()
            .map(|line| line.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{out}")
    }
}

fn count_mirror_mistakes<T>(vec: &[T]) -> usize
where
    T: PartialEq + Eq
{
    vec.iter()
        .enumerate()
        .filter(|(i, elem)| **elem != vec[vec.len()-i-1])
        .count()
    / 2
}

fn count_eq_mistakes<T>(vec_a: &[T], vec_b: &[T]) -> usize
where
    T: PartialEq + Eq
{
    vec_a.iter()
        .zip(vec_b.iter())
        .filter(|(a, b)| a != b)
        .count()
}

fn mirror_mistakes_nested<T>(vec: &[Vec<T>]) -> usize
where
    T: PartialEq + Eq
{
    vec.iter()
        .enumerate()
        .map(|(i, elem)| count_eq_mistakes(elem, &vec[vec.len()-i-1]))
        .sum::<usize>()
    / 2
}

impl Pattern {
    fn check_vert_reflect_subset(&self, x: usize, mistakes_allowed: usize) -> bool {
        let xmax = self.xmax();
        if x == 0 || x >= xmax {
            panic!("invalid reflection line: {x}");
        }

        let size = std::cmp::min(x, xmax - x);

        let subpattern = self.0.iter()
            .map(|line| line.get(x - size .. x + size).unwrap())
            .collect::<Vec<_>>();

        subpattern.iter()
            .map(|x| count_mirror_mistakes(x))
            .sum::<usize>() == mistakes_allowed
    }

    fn check_horiz_reflect_subset(&self, y: usize, mistakes_allowed: usize) -> bool {
        let ymax = self.ymax();
        if y == 0 || y >= ymax {
            panic!("invalid reflection line: {y} vs {ymax}");
        }

        let size = std::cmp::min(y, ymax - y);

        let subpattern = self.0.get(y - size..=y + size - 1).unwrap();

        mirror_mistakes_nested(subpattern) == mistakes_allowed
    }

    fn find_vert_reflect(&self, mistakes_allowed: usize) -> usize {
        (1..self.xmax())
            .find(|&i| self.check_vert_reflect_subset(i, mistakes_allowed))
            .unwrap_or(0)
    }

    fn find_horiz_reflect(&self, mistakes_allowed: usize) -> usize {
        (1..self.ymax())
            .find(|&i| self.check_horiz_reflect_subset(i, mistakes_allowed))
            .unwrap_or(0)
    }

    fn score(&self, mistakes_allowed: usize) -> usize {
        self.find_vert_reflect(mistakes_allowed) + self.find_horiz_reflect(mistakes_allowed) * 100
    }

    fn ymax(&self) -> usize {
        self.0.len()
    }

    fn xmax(&self) -> usize {
        self.0[0].len()
    }
}

fn a(data: &Vec<Pattern>) -> usize {
    data.iter()
        .map(|pattern| pattern.score(0))
        .sum()
}

fn b(data: &Vec<Pattern>) -> usize {
    data.iter()
        .map(|pattern| pattern.score(1))
        .sum()
}

fn main() {
    Puzzle {
        name: "13",
        parts: vec![a, b],
        delimiter: '\n',
        preprocess: |text| {
            let mut patterns = vec![];
            let mut cur = vec![];

            for line in text {
                if line.is_empty() {
                    patterns.push(Pattern(cur));
                    cur = vec![];
                    continue;
                }

                cur.push(line.chars().collect::<Vec<_>>());
            }

            patterns.push(Pattern(cur));

            patterns
        },
    }.solve();
}
