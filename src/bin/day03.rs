use regex::Regex;
use std::sync::LazyLock;

static MUL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());
static INSTRUCT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap());

fn part1(input: Vec<String>) -> usize {
    let mut result = 0;

    for line in input {
        for (_, [a, b]) in MUL_REGEX.captures_iter(&line).map(|c| c.extract()) {
            let a = a.parse::<usize>().unwrap();
            let b = b.parse::<usize>().unwrap();

            result += a * b;
        }
    }

    result
}

fn part2(input: Vec<String>) -> usize {
    let mut result = 0;
    let mut mul_on = true;

    for line in input {
        for m in INSTRUCT_REGEX.find_iter(&line).map(|m| m.as_str()) {
            match m {
                "do()" => {
                    mul_on = true;
                }
                "don't()" => {
                    mul_on = false;
                }
                _ => {
                    if !mul_on {
                        continue;
                    }

                    result += part1(vec![m.to_string()]);
                }
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

    fn get_fixture1() -> Vec<String> {
        vec!["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string()]
    }

    fn get_fixture2() -> Vec<String> {
        vec![
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        let fixture = get_fixture1();

        assert_eq!(part1(fixture), 161);
    }

    #[test]
    fn test_part2() {
        let fixture = get_fixture2();

        assert_eq!(part2(fixture), 48);
    }
}
