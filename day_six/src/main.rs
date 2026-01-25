use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let input_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("puzzle_input.txt");
    let homework: Vec<Vec<String>> = read_input(input_path);
    let expressions = solve_equations(&homework);
    println!("{:?}", expressions);
}

fn solve_equations(homework: &Vec<Vec<String>>) -> i64 {
    let mut answers: Vec<i64> = Vec::new();

    for col in 0..homework[0].len() {
        let mut expression: Vec<&String> = Vec::new();
        for row in 0..homework.len() {
            expression.push(&homework[row][col]);
        }

        let operator = expression.pop().expect("Missing operator!");
        let iter = expression.iter().map(|s| s.parse::<i64>().expect("Invalid integer."));
        match operator.as_str() {
            "+" => answers.push(iter.sum::<i64>()),
            "*" => answers.push(iter.product::<i64>()),
            &_ => ()
        };
    }

    answers.into_iter().sum()
}

fn read_input(path: PathBuf) -> Vec<Vec<String>> {
    let content = fs::read_to_string(path).expect("Couldn't read file.");
    let mut homework_matrix: Vec<Vec<String>> = Vec::new();
    for line in content.lines() {
        homework_matrix.push(
            line.split_whitespace()
                .map(String::from)
                .collect()
        );
    }

    homework_matrix
}
