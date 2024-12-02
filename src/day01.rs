use std::collections::HashMap;

pub fn part1(input: Vec<String>) -> usize {
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];

    for line in input {
        let parts = crate::parse_vec_from_line::<usize>(line).unwrap();
        left.push(parts[0]);
        right.push(parts[1]);
    }

    left.sort();
    right.sort();

    let mut result = 0;

    for i in 0..left.len() {
        if left[i] > right[i] {
            result += left[i] - right[i];
        } else {
            result += right[i] - left[i];
        }
    }

    result
}

pub fn part2(input: Vec<String>) -> usize {
    let mut right_frequency_map = HashMap::<usize, usize>::new();
    let mut left: Vec<usize> = vec![];

    for line in input {
        let parts: Vec<usize> = crate::parse_vec_from_line::<usize>(line).unwrap();
        left.push(parts[0]);
        *right_frequency_map.entry(parts[1]).or_insert(0) += 1;
    }

    let mut result = 0;

    for l in left {
        if let Some(&mut r) = right_frequency_map.get_mut(&l) {
            result += r * l;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        "3   4
        4   3
        2   5
        1   3
        3   9
        3   3"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        let fixture = get_fixture();

        assert_eq!(part1(fixture), 11);
    }

    #[test]
    fn test_part2() {
        let fixture = get_fixture();

        assert_eq!(part2(fixture), 31);
    }
}
