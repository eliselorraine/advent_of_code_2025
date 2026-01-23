use std::fs;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};

fn main() {
    let input_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("puzzle_input.txt");
    let (mut ranges, _ids) = read_input(input_path);
    ranges.sort_by_key(|r| *r.start());
    let merged = merge(&ranges);
    let mut count: u64 = 0;
    for range in merged {
        count += (range.end() - range.start()) + 1;
    }

    println!("{:?}", count);
}

fn merge(sorted_ranges: &Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    let mut merged: Vec<RangeInclusive<u64>> = Vec::new();
    let mut current: RangeInclusive<u64> = sorted_ranges[0].to_owned();
    for range in sorted_ranges {
        if range.start() < current.end() {
            let new_range = current.start()..=range.end();
            current = *current.start()..=*range.end();
            continue
        } 

        merged.push(current);
        current = range.clone();
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
}
