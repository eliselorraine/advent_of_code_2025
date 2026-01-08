use std::ops::RangeInclusive;

// iterators are lazy in rust
// this means we need to do something with it for it be called
// i.e. call collect, or sum, or for loop, or fold
// Strings are stored as a vector of u8 values

pub fn find_invalid_ids(range: RangeInclusive<u64>) -> Vec<u64> {
    let mut invalid_ids: Vec<u64> = vec![];
    for num in range {
        let stringified: String = num.to_string();
        if stringified.len() % 2 != 0 {
            continue
        }

        let midpoint = stringified.len() / 2;
        let invalid = &stringified[..midpoint] == &stringified[midpoint..];
        
        if invalid == true {
            invalid_ids.push(num);
        };
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
    assert_eq!(vec![99], result);
    Ok(())
}

#[test]
fn third_range() -> Result<(), Box<dyn std::error::Error>> {
    let range = 998..=1012;
    let result = find_invalid_ids(range.into());
    assert_eq!(vec![1010], result);
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
