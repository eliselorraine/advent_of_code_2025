use std::fs;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};

fn main() {
    let input_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("puzzle_input.txt");
    let (ranges, ids) = read_input(input_path);
    let mut fresh_ids = Vec::new();
    for (idx, id) in ids.into_iter().enumerate() {
        for range in &ranges {
            if range.contains(&id) {
                fresh_ids.push(idx); 
            }
        }    
    }

    fresh_ids.dedup();
    println!("{:?}", fresh_ids.len());
}

fn read_input(path: PathBuf) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let content: String = fs::read_to_string(path).expect("Couldn't read file.");
    let mut ranges: Vec<RangeInclusive<usize>> = Vec::new();
    let mut ids: Vec<usize> = Vec::new();

    for line in content.lines() {
        if line.is_empty() {
            continue;
        }
    
        if let Some((start, end)) = line.split_once('-') {
            ranges.push(
                start.parse::<usize>().unwrap()
                    ..= end.parse::<usize>().unwrap(),
            );
        } else {
            ids.push(line.parse::<usize>().unwrap());
        }
    }

    (ranges, ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_one() -> Result<(), Box<dyn std::error::Error>> {
        let input: RangeInclusive<usize> = 3..=5;
        let id: usize = 1;
        let result = check_id(input, 1);
        assert_eq!(false, result);
        Ok(())
    }
}
