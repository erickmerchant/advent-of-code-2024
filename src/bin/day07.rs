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

                if let Ok(parts) = advent::parse_vec_from_line::<usize>(parts.to_string()) {
                    let operator_combinations = operators
                        .iter()
                        .combinations_with_replacement(parts.len() - 1);

                    for combo in operator_combinations {
                        let operator_combinations = combo.iter().permutations(combo.len());

                        'combos: for combo in operator_combinations {
                            let mut t = parts[0];

                            for (i, operator) in combo.iter().enumerate() {
                                match operator {
                                    Operator::Multiply => t *= parts[i + 1],
                                    Operator::Add => t += parts[i + 1],
                                    Operator::Concat => {
                                        t = format!("{}{}", t, parts[i + 1]).parse().unwrap()
                                    }
                                }

                                if t > total {
                                    continue 'combos;
                                }
                            }

                            if t == total {
                                return total;
                            }
                        }
                    }
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
