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
        println!("{:?}", invalid_ids);
        sum += invalid_ids.into_iter().sum::<u64>();
    }

    println!("{}", sum);
}

fn read_input(path: std::path::PathBuf) -> Vec<RangeInclusive<u64>> {
    let content: String = fs::read_to_string(path).expect("Couldn't read file.");
 
    content.lines().map(|line| { 
        let ranges: Vec<RangeInclusive<u64>> = line.split(',').map(|r| { 
            let (start, end) = r.split_once('-').expect("Invalid range");
            start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
        }).collect();
        ranges
    }).flatten().collect()
}
