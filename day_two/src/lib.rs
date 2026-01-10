use std::ops::RangeInclusive;

fn check_if_duplicates(arr: Vec<&str>) -> bool {
    let pattern: &str = arr[0];
    
    arr.iter().all(|&x| x == pattern)
}

pub fn find_invalid_ids(range: RangeInclusive<usize>) -> Vec<usize> {
    let mut invalid_ids: Vec<usize> = vec![];
    for num in range {
        let stringified: String = num.to_string();
        let mut midpoint = stringified.len() / 2;
        for size in 1..=midpoint {
            let vec: Vec<&str> = stringified.as_bytes()
                .chunks(size)
                .map(|chunk| std::str::from_utf8(chunk).unwrap())
                .collect();

            if check_if_duplicates(vec) {
                invalid_ids.push(num);
                break
            }
        }
    }

    invalid_ids
}

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
fn check_if_duplicates_works() -> Result<(), Box<dyn std::error::Error>> {
    let arr = vec!["1", "1", "1"];
    let result = check_if_duplicates(arr);
    assert_eq!(true, result);
    Ok(())
}

#[test]
fn check_if_duplicates_works_still() -> Result<(), Box<dyn std::error::Error>> {
    let arr = vec!["1", "1", "2", "1"];
    let result = check_if_duplicates(arr);
    assert_eq!(false, result);
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
