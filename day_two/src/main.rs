use clap::Parser;
use std::fs;
use std::ops::RangeInclusive;

use day_two::find_invalid_ids;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let ranges = read_input(args.path);
    let mut sum = 0;
    for range in ranges {
        let invalid_ids = find_invalid_ids(range);
        // println!("{:?}", invalid_ids);
        sum += invalid_ids.into_iter().sum::<usize>();
    }

    println!("{}", sum);
}

fn read_input(path: std::path::PathBuf) -> Vec<RangeInclusive<usize>> {
    let content: String = fs::read_to_string(path).expect("Couldn't read file.");
 
    content.lines().map(|line| { 
        let ranges: Vec<RangeInclusive<usize>> = line.split(',').map(|r| { 
            let (start, end) = r.split_once('-').expect("Invalid range");
            start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap()
        }).collect();
        ranges
    }).flatten().collect()
}
