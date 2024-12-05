use itertools::Itertools;
use std::cmp::Ordering;

type Rule = (usize, usize);

fn part1(input: Vec<String>) -> usize {
    let mut rules: Vec<Rule> = vec![];
    let mut result = 0;

    'outer: for line in input {
        if line.contains('|') {
            let rule: Rule = line
                .split('|')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();

            rules.push(rule);
        }

        if line.contains(',') {
            let numbers: Vec<usize> = line
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            for (a, b) in rules.clone() {
                match (
                    numbers.iter().position(|&x| x == a),
                    numbers.iter().position(|&x| x == b),
                ) {
                    (Some(a), Some(b)) if a > b => {
                        continue 'outer;
                    }
                    _ => {}
                };
            }

            result += numbers[numbers.len() / 2]
        }
    }

    result
}

fn part2(input: Vec<String>) -> usize {
    let mut rules: Vec<Rule> = vec![];
    let mut result = 0;

    'outer: for line in input {
        if line.contains('|') {
            let rule: Rule = line
                .split('|')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();

            rules.push(rule);
        }

        if line.contains(',') {
            let mut numbers: Vec<usize> = line
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            for (a, b) in rules.clone() {
                match (
                    numbers.iter().position(|&x| x == a),
                    numbers.iter().position(|&x| x == b),
                ) {
                    (Some(a), Some(b)) if a > b => {
                        numbers.sort_by(|x, y| {
                            for (a, b) in rules.clone() {
                                if x == &a && y == &b {
                                    return Ordering::Less;
                                }

                                if x == &b && y == &a {
                                    return Ordering::Greater;
                                }
                            }

                            Ordering::Equal
                        });

                        result += numbers[numbers.len() / 2];

                        continue 'outer;
                    }
                    _ => {}
                };
            }
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
        "47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        let fixture = get_fixture();

        assert_eq!(part1(fixture), 143);
    }

    #[test]
    fn test_part2() {
        let fixture = get_fixture();

        assert_eq!(part2(fixture), 123);
    }
}
