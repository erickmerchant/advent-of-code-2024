use rayon::prelude::*;

fn get_sum(stones: &[usize], iterations: usize) -> usize {
    if iterations == 0 {
        return stones.len();
    }

    stones
        .par_iter()
        .map(|stone| {
            if *stone == 0 {
                return get_sum(&[1], iterations - 1);
            }

            let length = stone.ilog10() + 1;

            if length % 2 == 0 {
                let left = stone / 10usize.pow(length / 2);
                let right = stone % 10usize.pow(length / 2);

                return get_sum(&[left, right], iterations - 1);
            }

            get_sum(&[stone * 2024], iterations - 1)
        })
        .sum()
}

fn get_result(input: String, iterations: usize) -> usize {
    let stones: Vec<usize> = input
        .split_whitespace()
        .map(|p| p.parse().unwrap())
        .collect();

    get_sum(&stones, iterations)
}

fn part1(input: String) -> usize {
    get_result(input, 25)
}

fn main() {
    let input = advent::get_input();
    let input = input.first().unwrap();

    println!("{}", part1(input.clone()));
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
