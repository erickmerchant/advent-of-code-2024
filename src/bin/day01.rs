use std::collections::HashMap;

fn part1(input: Vec<String>) -> usize {
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();

    for line in input {
        let parts: Vec<usize> = line
            .split_whitespace()
            .map(|p| p.parse().unwrap())
            .collect();

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

fn part2(input: Vec<String>) -> usize {
    let mut right_frequency_map: HashMap<usize, usize> = HashMap::new();
    let mut left: Vec<usize> = Vec::new();

    for line in input {
        let parts: Vec<usize> = line
            .split_whitespace()
            .map(|p| p.parse().unwrap())
            .collect();

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

fn main() {
    let input = advent::get_input();

    println!("{}", part1(input.clone()));
    println!("{}", part2(input.clone()));
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
