use std::fs;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};

fn main() {
    let input_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("test_input.txt");
    let (mut ranges, _ids) = read_input(input_path);
    ranges.sort_by_key(|r| *r.start());
    let merged = merge(&ranges);
    let count: u64 = count(&merged);

    println!("{:?}", count);
}

fn count(merged_ranges: &Vec<RangeInclusive<u64>>) -> u64 {
    let mut count: u64 = 0;
    for range in merged_ranges {
        count += (range.end() - range.start()) + 1;
    }

    count
}

fn merge(sorted_ranges: &Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    let mut merged: Vec<RangeInclusive<u64>> = Vec::new();
    let mut current: RangeInclusive<u64> = sorted_ranges[0].to_owned();

    for range in sorted_ranges.iter().skip(1) {
        if range.start() <= current.end() {
            let new_end = (current.end()).max(range.end());
            current = *current.start()..=*new_end;
        } else {
            merged.push(current);
            current = range.clone();
        }
    }
    
    merged.push(current);
    merged
}

fn read_input(path: PathBuf) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let content: String = fs::read_to_string(path).expect("Couldn't read file.");
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();

    for line in content.lines() {
        if line.is_empty() {
            continue;
        }
    
        if let Some((start, end)) = line.split_once('-') {
            ranges.push(
                start.parse::<u64>().unwrap()
                    ..= end.parse::<u64>().unwrap(),
            );
        } else {
            ids.push(line.parse::<u64>().unwrap());
        }
    }

    (ranges, ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_ranges() -> Result<(), Box<dyn std::error::Error>> {
        let ranges: Vec<RangeInclusive<u64>> = vec![3..=5, 4..=6, 10..=20];
        let result = merge(&ranges);
        assert_eq!(vec![3..=6, 10..=20], result);
        Ok(())
    }

    #[test]
    fn merge_more_small_ranges() -> Result<(), Box<dyn std::error::Error>> {
        let ranges: Vec<RangeInclusive<u64>> = vec![3..=5, 10..=14, 14..=18, 16..=20];
        let result = merge(&ranges);
        assert_eq!(vec![3..=5, 10..=20], result);
        Ok(())
    }

    #[test]
    fn count_more_small_ranges() -> Result<(), Box<dyn std::error::Error>> {
        let ranges: Vec<RangeInclusive<u64>> = vec![3..=5, 10..=14, 12..=18, 16..=20];
        let merged = merge(&ranges);
        let result = count(&merged);
        assert_eq!(14, result);
        Ok(())
    }
    
    #[test]
    fn merge_larger_ranges() -> Result<(), Box<dyn std::error::Error>> {
        let ranges: Vec<RangeInclusive<u64>> = vec![307684522928..=1110109620128, 794175129155..=1456801270414];
        let result = merge(&ranges);
        assert_eq!(vec![307684522928..=1456801270414], result);
        Ok(())
    }
    
    #[test]
    fn count_small_ranges() -> Result<(), Box<dyn std::error::Error>> {
        let ranges: Vec<RangeInclusive<u64>> = vec![3..=5, 4..=6, 10..=20];
        let merged = merge(&ranges);
        let result = count(&merged);
        assert_eq!(15, result);
        Ok(())
    }
}
