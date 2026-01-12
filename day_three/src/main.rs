use std::fs;

fn main() {
    let banks = read_file("puzzle_input.txt");
    let mut sum = 0;
    for bank in banks {
        let highest_voltage = get_highest_voltage(&bank);
        sum += highest_voltage;
    }

    println!("{:?}", sum);
}

fn get_first_digit(bank: &[u32]) -> (usize, &u32) {
    let max = get_max(&bank);
    let idx = bank.iter().position(|x| x == max).unwrap();
    (idx, max)
}

fn get_max(bank: &[u32]) -> &u32 {
    bank.iter().max().unwrap()
}

fn get_highest_voltage(bank: &Vec<u32>) -> u32 {
    let (idx, first_digit) = get_first_digit(&bank[..(bank.len() - 1)]);
    let second_digit = get_max(&bank[(idx + 1)..]);
    let highest_voltage = (first_digit * 10) + second_digit;

    highest_voltage
}

fn read_file(path: &str) -> Vec<Vec<u32>> {
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
        let input: Vec<u32> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];
        let result = get_highest_voltage(&input);
        assert_eq!(98, result);
        Ok(())
    }

    #[test]
    fn bank_two() -> Result<(), Box<dyn std::error::Error>> {
        let input: Vec<u32> = vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9];
        let result = get_highest_voltage(&input);
        assert_eq!(89, result);
        Ok(())
    }

    #[test]
    fn bank_three() -> Result<(), Box<dyn std::error::Error>> {
        let input: Vec<u32> = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
        let result = get_highest_voltage(&input);
        assert_eq!(78, result);
        Ok(())
    }

    #[test]
    fn bank_four() -> Result<(), Box<dyn std::error::Error>> {
        let input: Vec<u32> = vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1];
        let result = get_highest_voltage(&input);
        assert_eq!(92, result);
        Ok(())
    }
    
    #[test]
    fn bank_five() -> Result<(), Box<dyn std::error::Error>> {
        let input: Vec<u32> = vec![8, 4, 5, 7, 8, 5];
        let (idx, first_digit) = get_first_digit(&input[..(&input.len() - 1)]);
        let second_digit = get_max(&input[(idx + 1)..]);
        let voltage = get_highest_voltage(&input);
        assert_eq!(idx, 0);
        assert_eq!(&8, first_digit);
        assert_eq!(&8, second_digit);
        assert_eq!(88, voltage);
        Ok(())
    }
}
