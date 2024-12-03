advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut good_reports: u32 = 0;
    for line in input.lines() {
        let levels = line
            .split(" ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        if is_report_safe(&levels) {
            good_reports += 1;
        }
    }
    Some(good_reports)
}

fn is_report_safe(levels: &Vec<u32>) -> bool {
    let mut diffs = Vec::<i32>::new();
    for i in 0..levels.len() - 1 {
        let diff = levels[i] as i32 - levels[i + 1] as i32;
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        diffs.push(diff);
    }
    let filtered_diff_count = diffs.iter().filter(|&x| *x > 0).count();
    if filtered_diff_count == diffs.len() || filtered_diff_count == 0 {
        return true;
    }
    false
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut good_reports: u32 = 0;
    let mut bad_reports = Vec::<Vec<u32>>::new();
    for line in input.lines() {
        let levels = line
            .split(" ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        if is_report_safe(&levels) {
            good_reports += 1;
        } else {
            bad_reports.push(levels);
        }
    }
    for levels in bad_reports {
        for i in 0..levels.len() {
            let mut new_levels = levels.clone();
            new_levels.remove(i);
            if is_report_safe(&new_levels) {
                good_reports += 1;
                break;
            }
        }
    }
    Some(good_reports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
