use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_name = "./day_2_input.txt";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut total_paper = 0;
    let mut total_ribbon = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        //println!("{}. {}", index + 1, line);

        let mut split_iter = line.splitn(3, 'x');
        let l: i32 = split_iter.next().expect("Expected a number").parse().unwrap();
        let b: i32 = split_iter.next().expect("Expected a number").parse().unwrap();
        let w: i32 = split_iter.next().expect("Expected a number").parse().unwrap();
        
        //println!("{l}, {b}, {w}");

        let mut faces = Vec::new();
        faces.push(l * b);
        faces.push(l * w);
        faces.push(b * w);

        faces.sort();

        //println!("{}, {}, {}", &faces[0], &faces[1], &faces[2]);

        total_paper += 3 * &faces[0] + 2 * &faces[1] + 2 * &faces[2];

        let mut sides = Vec::new();
        sides.push(l);
        sides.push(b);
        sides.push(w);

        sides.sort();
        
        total_ribbon += &sides[0] * 2 + &sides[1] * 2 + (l * b * w);
    }

    println!("Paper required: {total_paper} square feet.");
    println!("Ribbon required: {total_ribbon} feet.");
    
}
