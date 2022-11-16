use std::{fs, collections::HashMap};

fn main() {
    let day_3_input = fs::read_to_string("./day_3_input.txt")
        .expect("Should have been able to read the file.");

    let directions= day_3_input.as_str();
    let mut current_pos = Pos{x: 0, y: 0};
    let mut houses: HashMap<Pos, u8> = HashMap::new();

    for direction in directions.chars() {
        current_pos = process_direction(direction, current_pos);
        let count = houses.entry(current_pos).or_insert(0);
        *count += 1;
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