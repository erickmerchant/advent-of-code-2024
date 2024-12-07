use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Clone)]
enum Operator {
    Multiply,
    Add,
    Concat,
}

fn part1(input: Vec<String>) -> usize {
    get_result(input, &[Operator::Multiply, Operator::Add])
}

fn part2(input: Vec<String>) -> usize {
    get_result(
        input,
        &[Operator::Multiply, Operator::Add, Operator::Concat],
    )
}

fn get_result(input: Vec<String>, operators: &[Operator]) -> usize {
    input
        .par_iter()
        .map(|line| {
            if let Some((total, parts)) = line.split(":").collect_tuple::<(&str, &str)>() {
                let total = total.trim().parse::<usize>().unwrap();
                let parts = parts.split_whitespace().collect::<Vec<&str>>();
                let mut results = vec![parts[0].parse::<usize>().unwrap()];

                for part in &parts[1..parts.len()] {
                    let mut new_results = vec![];

                    for result in results {
                        for operator in operators {
                            let new_result = match operator {
                                Operator::Multiply => result * part.parse::<usize>().unwrap(),
                                Operator::Add => result + part.parse::<usize>().unwrap(),
                                Operator::Concat => {
                                    result * 10_usize.pow(part.len() as u32)
                                        + part.parse::<usize>().unwrap()
                                }
                            };

                            if new_result <= total {
                                new_results.push(new_result);
                            }
                        }
                    }

                    results = new_results;
                }

                if results.contains(&total) {
                    return total;
                }
            }

            0
        })
        .sum()
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
        "190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        let fixture = get_fixture();

        assert_eq!(part1(fixture), 3749);
    }

    #[test]
    fn test_part2() {
        let fixture = get_fixture();

        assert_eq!(part2(fixture), 11387);
    }
}
