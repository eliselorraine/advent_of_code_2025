use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rotations = read_file("puzzle_input.txt");
    let mut pointer = 50;
    let mut count = 0;
    
    for rotation in rotations {
        pointer = pointer + rotation;
        (pointer, count) = turn_dial(pointer, count);
    }

    println!("count: {}", count);
    println!("dial at: {}", pointer);
    
    Ok(())
}

fn read_file(path: &str) -> Vec<i32> {
    let content: String = fs::read_to_string(path).expect("Couldn't read file.");
    let mut rotations: Vec<i32> = vec![];
    for line in content.lines() {
        let (direction, rest) = line.split_at(1);
        let amount: i32 = rest.parse().expect("Invalid integer in rotation.");
        let rotation: i32 = match direction {
            "R" => amount,
            "L" => -amount,
            _ => 0
        };
        rotations.push(rotation)
    }

    rotations
}

fn turn_dial(mut pointer: i32, mut count: u32) -> (i32, u32) {
    while pointer < 0 {
        pointer = 100 + pointer
    }
    
    while pointer > 99 {
        pointer = pointer - 100
    }

    if pointer == 0 {
        count += 1
    }

    (pointer, count)
}
