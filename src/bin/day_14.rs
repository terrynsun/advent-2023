use advent_2023::puzzle::Puzzle;
use advent_2023::twod::Coord;
use advent_2023::twod::Map;

fn get_first_northern_rock(map: &Map, x: i32, y: i32) -> Coord {
    let mut final_coord = Coord { x, y };

    // this could be a weird fold or a search
    for i in (0..y).rev() {
        let target_obj = map.get(Coord { x, y: i }).unwrap();
        match target_obj  {
            // ok to move here
            '.' => final_coord = Coord { x , y: i },
            // roadblock, return as far as we've slid
            'O' | '#' => {
                return final_coord;
            },
            _ => panic!("invalid symbol"),
        }
    }

    final_coord
}

fn a(map: &Map) -> usize {
    let mut map = map.clone();
    println!("{map}");
    for (y, line) in map.data.clone().iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == 'O' {
                let new_coord = get_first_northern_rock(&map, x as i32, y as i32);
                map.set(Coord::from_usize(x, y), '.');
                map.set(new_coord, 'O');
            }
        }
    }

    println!("\n{map}");
    map.data.iter().enumerate()
        .map(|(y, line)| { line.iter().filter(|&&c| c == 'O').count() * (map.ymax as usize - y)})
        .sum::<usize>()
}

fn b(_data: &Map) -> usize {
    0
}

fn main() {
    Puzzle {
        name: "14",
        parts: vec![a, b],
        delimiter: '\n',
        preprocess: Map::from_strings,
    }.solve();
}
