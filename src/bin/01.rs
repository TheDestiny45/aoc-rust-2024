use std::fs;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut vec1: Vec<u32> = Vec::new();
    let mut vec2: Vec<u32> = Vec::new();
    for line in input.lines() {
        let parts = line.split_once("   ").unwrap();
        vec1.push(parts.0.parse::<u32>().unwrap());
        vec2.push(parts.1.parse::<u32>().unwrap());
    }
    vec1.sort();
    vec2.sort();
    let mut diff = Vec::new();
    for i in 0..vec1.len() {
        diff.push(vec2[i].abs_diff(vec1[i]));
    }
    Some(diff.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut vec1: Vec<u32> = Vec::new();
    let mut vec2: Vec<u32> = Vec::new();
    for line in input.lines() {
        let parts = line.split_once("   ").unwrap();
        vec1.push(parts.0.parse::<u32>().unwrap());
        vec2.push(parts.1.parse::<u32>().unwrap());
    }
    let mut similarity: u32 = 0;
    for element in vec1 {
        similarity += element * vec2.iter().filter(|x| **x == element).count() as u32;
    }
    Some(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
