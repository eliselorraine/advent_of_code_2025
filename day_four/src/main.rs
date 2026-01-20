use std::fs;

fn main() {
    let grid = read_input("puzzle_input.txt");
    let mut total: usize = 0;

    total = check_grid(&grid, total);

    println!("{}", total);
}

fn remove_rolls(grid: &Vec<Vec<char>>, remove_roll_coords: Vec<(usize, usize)>) -> Vec<Vec<char>>{
    let mut new_grid = grid.clone();

    for coord in remove_roll_coords {
        new_grid[coord.0][coord.1] = '.';
    }

    new_grid
}

fn check_grid(grid: &Vec<Vec<char>>, mut running_total: usize) -> usize {
    let mut removable_rolls: Vec<(usize, usize)> = Vec::new();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '@' {
                let count = look_around(&grid, row, col);
                if count < 4 {
                    running_total += 1;
                    removable_rolls.push((row, col));
                }
            }
        }
    }
    
    if removable_rolls.len() > 0 {
        let cleaned_grid = remove_rolls(&grid, removable_rolls);
        return check_grid(&cleaned_grid, running_total)
    } 
  
    running_total
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
