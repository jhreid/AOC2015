use std::fs;

fn main() {
    let day_1_input = fs::read_to_string("./day_1_input.txt")
        .expect("Should have been able to read the file.");

    let mut floor = 0;
    let brackets = day_1_input.as_bytes();

    for (i, &bracket) in brackets.iter().enumerate() {
        if bracket == b'(' {
            floor += 1
        } else if bracket == b')' {
            floor -= 1
        }
        if floor == -1 {
            println!("In the basement at {}", i + 1);
        }
    }

    println!("Ended up on floor {floor}");
}