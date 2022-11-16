use std::{fs, collections::HashMap};

fn main() {
    let day_3_input = fs::read_to_string("./day_3_input.txt")
        .expect("Should have been able to read the file.");

    let directions= day_3_input.as_str();
    let mut santa_pos = Pos{x: 0, y: 0};
    let mut robo_pos = Pos{x: 0, y: 0};
    let mut houses: HashMap<Pos, u8> = HashMap::new();

    houses.insert(santa_pos, 1);

    for (index, direction) in directions.chars().enumerate() {
        if index % 2 == 0 {
            santa_pos = process_direction(direction, santa_pos);
            let santa_count = houses.entry(santa_pos).or_insert(0);
            *santa_count += 1;
        } else {
            robo_pos = process_direction(direction, robo_pos);
            let robo_count = houses.entry(robo_pos).or_insert(0);
            *robo_count += 1;
        }
    }
    println!("Houses visited: {}", houses.len());
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

fn process_direction(direction: char, position: Pos) -> Pos {
    match direction {
        '<' => Pos {x: position.x - 1, y: position.y, },
        '>' => Pos {x: position.x + 1, y: position.y, },
        '^' => Pos {x: position.x, y: position.y + 1, },
        'v' => Pos {x: position.x, y: position.y - 1, },
        _ => position,
    }
} 