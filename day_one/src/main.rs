use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rotations = read_file("puzzle_input.txt");
    let mut pointer = 50;
    let mut count = 0;

    for rotation in rotations {
        let distance_to_zero = distance_to_zero(rotation, pointer);
        if rotation.abs() > distance_to_zero {
            if distance_to_zero != 0 {
                count += 1;
            }

            let full_rotations = (rotation.abs() - distance_to_zero) / 100; 
            count += full_rotations;
            pointer = turn_dial(pointer, &rotation);
        } else {
            pointer = turn_dial(pointer, &rotation);
            if pointer == 0 {
                count += 1;
            }
        }
    }
    
    println!("count: {}", count);
    println!("dial at: {}", pointer);

    Ok(())
}

fn distance_to_zero(rotation: i32, pointer: i32) -> i32 {
    match rotation > 0 {
        true => 100 - pointer,
        false => pointer
    }
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

fn turn_dial(mut pointer: i32, rotation: &i32) -> i32 {
    pointer = pointer + rotation;

    while pointer < 0 {
        pointer = 100 + pointer
    }
    
    while pointer > 99 {
        pointer = pointer - 100
    }

    pointer
}

// 6496 // dial 93 is correct answer
