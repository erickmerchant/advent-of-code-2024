use std::collections::HashMap;

fn get_sum(stone: usize, iterations: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(&result) = cache.get(&(stone, iterations)) {
        return result;
    }

    let result = {
        if iterations == 0 {
            return 1;
        }

        if stone == 0 {
            return get_sum(1, iterations - 1, cache);
        }

        let length = stone.ilog10() + 1;

        if length % 2 == 0 {
            let left = stone / 10usize.pow(length / 2);
            let right = stone % 10usize.pow(length / 2);

            return get_sum(left, iterations - 1, cache) + get_sum(right, iterations - 1, cache);
        }

        get_sum(stone * 2024, iterations - 1, cache)
    };

    cache.insert((stone, iterations), result);

    result
}

fn get_result(input: String, iterations: usize) -> usize {
    let stones: Vec<usize> = input
        .split_whitespace()
        .map(|p| p.parse().unwrap())
        .collect();

    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

    stones
        .iter()
        .map(|stone| get_sum(*stone, iterations, &mut cache))
        .sum()
}

fn part1(input: String) -> usize {
    get_result(input, 25)
}

fn part2(input: String) -> usize {
    get_result(input, 75)
}

fn main() {
    let input = advent::get_input();
    let input = input.first().unwrap();

    println!("{}", part1(input.clone()));
    println!("{}", part2(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> String {
        "125 17".to_string()
    }

    #[test]
    fn test_get_result() {
        let fixture = get_fixture();

        assert_eq!(get_result(fixture, 25), 55312);
    }
}
