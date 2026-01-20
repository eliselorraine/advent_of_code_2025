use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let input_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("puzzle_input.txt");
    let banks = read_file(input_path);
    let mut sum: u64 = 0;
    for bank in banks {
        let bank_sum: u64 = get_highest_voltage(&bank);
        sum += bank_sum;
    }

    println!("{:?}", sum);
}

fn get_highest_voltage(bank: &[u32]) -> u64 {
    let mut remaining_iterations: u8 = 12;
    let mut slice_idx: usize = 11;
    let mut sum: u64 = 0;
    let mut start_slice_idx: usize = 0;
    let enumerated_bank: Vec<(usize, u32)> = bank
        .iter()
        .enumerate()
        .map(|(idx, int)| (idx, *int))
        .collect();

    while remaining_iterations > 0 {
        remaining_iterations -= 1;

        let end_slice_idx: usize = bank.len() - slice_idx;
        let slice = &enumerated_bank[start_slice_idx..end_slice_idx];
        let (idx, digit) = get_max(slice);
        sum += (digit as u64) * 10_u64.pow(slice_idx.try_into().unwrap());

        if slice_idx > 0 {
            slice_idx -= 1;
        }

        start_slice_idx = idx + 1;
    }

    sum
}

fn get_max(bank: &[(usize, u32)]) -> (usize, u32) {
    let mut max = bank[0];
    for &t in bank.iter() {
        if t.1 > max.1 {
            max = t;
        }
    }

    max
}

fn read_file(path: PathBuf) -> Vec<Vec<u32>> {
    let content: String = fs::read_to_string(path).expect("Couldn't read file.");
    let mut banks: Vec<Vec<u32>> = vec![];
    for line in content.lines() {
        let bank: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        banks.push(bank)
    }

    banks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bank_one() -> Result<(), Box<dyn std::error::Error>> {
        let input: Vec<u32> = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
        let result = get_highest_voltage(&input);
        assert_eq!(434234234278, result);
        Ok(())
    }

    #[test]
    fn bank_two() -> Result<(), Box<dyn std::error::Error>> {
        let input: Vec<u32> = vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9];
        let result = get_highest_voltage(&input);
        assert_eq!(811111111119, result);
        Ok(())
    }

    #[test]
    fn bank_three() -> Result<(), Box<dyn std::error::Error>> {
        let input: Vec<u32> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];
        let result = get_highest_voltage(&input);
        assert_eq!(987654321111, result);
        Ok(())
    }

    #[test]
    fn bank_four() -> Result<(), Box<dyn std::error::Error>> {
        let input: Vec<u32> = vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1];
        let result = get_highest_voltage(&input);
        assert_eq!(888911112111, result);
        Ok(())
    }
}
