use std::fs;
use std::ops::RangeInclusive;

fn main() {
    let ranges = read_input("puzzle_input.txt".into());
    let mut sum = 0;
    for range in ranges {
        let invalid_ids = find_invalid_ids(range);
        sum += invalid_ids.iter().sum::<usize>();
    }

    println!("{}", sum);
}

fn read_input(path: std::path::PathBuf) -> Vec<RangeInclusive<usize>> {
    let content: String = fs::read_to_string(path).expect("Couldn't read file.");

    content
        .lines()
        .map(|line| {
            let ranges: Vec<RangeInclusive<usize>> = line
                .split(',')
                .map(|r| {
                    let (start, end) = r.split_once('-').expect("Invalid range");
                    start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap()
                })
                .collect();
            ranges
        })
        .flatten()
        .collect()
}

fn check_if_duplicates(arr: Vec<usize>) -> bool {
    let pattern: usize = arr[0];

    arr.iter().all(|&x| x == pattern)
}


fn num_digits(mut n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut digits = 0;
    while n > 0 {
        digits += 1;
        n /= 10;
    }
    digits
}

fn split_number(mut n: usize, chunk_size: usize) -> Vec<usize> {
    let base = 10usize.pow(chunk_size as u32);
    let mut chunks = Vec::new();

    while n > 0 {
        chunks.push(n % base);
        n /= base;
    }

    chunks.reverse();
    chunks
}

fn find_invalid_ids(range: RangeInclusive<usize>) -> Vec<usize> {
    let mut invalid_ids: Vec<usize> = vec![];
    for num in range {
        let digits = num_digits(num);
        let midpoint = digits / 2;

        for size in 1..=midpoint {
            let chunks = split_number(num, size);
            if check_if_duplicates(chunks) {
                invalid_ids.push(num);
                break
            }
        }
    }

    invalid_ids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_range() -> Result<(), Box<dyn std::error::Error>> {
        let range = 11..=22;
        let result = find_invalid_ids(range.into());
        assert_eq!(vec![11, 22], result);
        Ok(())
    }

    #[test]
    fn second_range() -> Result<(), Box<dyn std::error::Error>> {
        let range = 95..=115;
        let result = find_invalid_ids(range.into());
        assert_eq!(vec![99, 111], result);
        Ok(())
    }

    #[test]
    fn third_range() -> Result<(), Box<dyn std::error::Error>> {
        let range = 998..=1012;
        let result = find_invalid_ids(range.into());
        assert_eq!(vec![999, 1010], result);
        Ok(())
    }

    #[test]
    fn fourth_range() -> Result<(), Box<dyn std::error::Error>> {
        let range = 1188511880..=1188511890;
        let result = find_invalid_ids(range.into());
        assert_eq!(vec![1188511885], result);
        Ok(())
    }

    #[test]
    fn fifth_range() -> Result<(), Box<dyn std::error::Error>> {
        let range = 222220..=222224;
        let result = find_invalid_ids(range.into());
        assert_eq!(vec![222222], result);
        Ok(())
    }

    #[test]
    fn duplicates_returns_true() -> Result<(), Box<dyn std::error::Error>> {
        let arr = vec![1, 1, 1];
        let result = check_if_duplicates(arr);
        assert_eq!(true, result);
        Ok(())
    }

    #[test]
    fn duplicates_returns_false() -> Result<(), Box<dyn std::error::Error>> {
        let arr = vec![123, 23, 123, 123];
        let result = check_if_duplicates(arr);
        assert_eq!(false, result);
        Ok(())
    }
}
