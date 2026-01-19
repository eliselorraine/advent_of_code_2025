use std::fs;

fn main() {
    let grid = read_input("puzzle_input.txt");
    let mut total: u32 = 0;
    for (row, line) in grid.iter().enumerate() {
        for (col, item) in line.iter().enumerate() {
            if *item == '@' {
                let count = look_around(&grid, row, col);
                if count < 4 {
                    total += 1
                }
            }
        }
    }

    println!("{}", total);
}

fn look_around(grid: &Vec<Vec<char>>, row: usize, col: usize) -> u8 {
    let total_rows = grid.len() - 1;
    let total_col = grid[0].len() - 1;
    let mut count: u8 = 0;

    // right
    if col != total_col && grid[row][col + 1] == '@' {
        count += 1
    }

    // left
    if col != 0 && grid[row][col - 1] == '@' {
        count += 1;
    }

    // above
    if row != 0 && grid[row - 1][col] == '@' {
        count += 1;
    }
    
    // above right
    if col != total_col && row != 0 && grid[row - 1][col + 1] == '@' {
        count += 1;
    }

    // above left
    if col != 0 && row != 0 && grid[row - 1][col - 1] == '@' {
        count += 1;
    }

    // below
    if row != total_rows && grid[row + 1][col] == '@' {
        count += 1;
    }

    // below right
    if row != total_rows && col != total_col && grid[row + 1][col + 1] == '@' {
        count += 1;
    }

    // below left   
    if row != total_rows && col != 0 && grid[row + 1][col - 1] == '@' { 
        count += 1;
    }
    
    count
}

fn read_input(path: &str) -> Vec<Vec<char>> {
    let content = fs::read_to_string(path).expect("Couldn't read file.");
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in content.lines() {
        grid.push(line.chars().collect());
    }

    grid
}
