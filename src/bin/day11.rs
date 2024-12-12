use num::{BigInt, FromPrimitive};
use rayon::prelude::*;

fn get_sum(stones: &[BigInt], iterations: usize) -> usize {
    if iterations == 0 {
        return stones.len();
    }

    stones
        .par_iter()
        .map(|stone| {
            if *stone == BigInt::ZERO {
                return get_sum(&[FromPrimitive::from_usize(1).unwrap()], iterations - 1);
            }

            let stone_as_string = stone.to_str_radix(10);

            if stone_as_string.len() % 2 == 0 {
                let (a, b) = stone_as_string.split_at(stone_as_string.len() / 2);

                return get_sum(&[a.parse().unwrap(), b.parse().unwrap()], iterations - 1);
            }

            get_sum(&[stone * 2024], iterations - 1)
        })
        .sum()
}

fn get_result(input: String, iterations: usize) -> usize {
    let stones: Vec<BigInt> = input
        .split_whitespace()
        .map(|p| p.parse().unwrap())
        .collect();

    get_sum(&stones, iterations)
}

fn part1(input: String) -> usize {
    get_result(input, 25)
}

// fn part2(input: String) -> usize {
//     get_result(input, 75)
// }

fn main() {
    let input = advent::get_input();
    let input = input.first().unwrap();

    println!("{}", part1(input.clone()));
    // println!("{}", part2(input.clone()));
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
