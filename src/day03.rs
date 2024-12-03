use once_cell::sync::Lazy;
use regex::Regex;

pub fn part1(input: Vec<String>) -> usize {
    static MUL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());
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

pub fn part2(input: Vec<String>) -> usize {
    static INSTRUCT_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap());
    static MUL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());
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

                    for (_, [a, b]) in MUL_REGEX.captures_iter(m).map(|c| c.extract()) {
                        let a = a.parse::<usize>().unwrap();
                        let b = b.parse::<usize>().unwrap();

                        result += a * b;
                    }
                }
            }
        }
    }

    result
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
