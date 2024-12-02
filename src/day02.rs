fn test_parts(parts: Vec<isize>) -> usize {
    let mut parts = parts.clone();

    if !parts.is_sorted() {
        parts.reverse();

        if !parts.is_sorted() {
            return 0;
        }
    }

    let mut prev = None;

    for part in parts {
        prev = match prev {
            None => Some(part),
            Some(prev) => {
                let diff = (prev - part).abs();

                if !(1..=3).contains(&diff) {
                    return 0;
                }

                Some(part)
            }
        };
    }

    1
}

pub fn part1(input: Vec<String>) -> usize {
    let mut result = 0;

    for line in input {
        let parts = crate::parse_vec_from_line::<isize>(line).unwrap();

        result += test_parts(parts);
    }

    result
}

pub fn part2(input: Vec<String>) -> usize {
    let mut result = 0;

    'outer: for line in input {
        let parts = crate::parse_vec_from_line::<isize>(line).unwrap();

        let r = test_parts(parts.clone());

        result += r;

        if r == 1 {
            continue 'outer;
        }

        for i in 0..parts.len() {
            let mut parts = parts.clone();
            parts.remove(i);

            let r = test_parts(parts);

            result += r;

            if r == 1 {
                continue 'outer;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        let fixture = get_fixture();

        assert_eq!(part1(fixture), 2);
    }

    #[test]
    fn test_part2() {
        let fixture = get_fixture();

        assert_eq!(part2(fixture), 4);
    }
}
