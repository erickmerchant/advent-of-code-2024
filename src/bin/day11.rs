use num::{BigInt, FromPrimitive};
use rayon::prelude::*;

fn get_result(input: String, iterations: usize) -> usize {
    let mut stones: Vec<BigInt> = input
        .split_whitespace()
        .map(|p| FromPrimitive::from_usize(p.parse::<usize>().unwrap()).unwrap())
        .collect();

    for _i in 0..iterations {
        let new_stones: Vec<BigInt> = stones
            .clone()
            .par_iter()
            .map(|stone| {
                let mut new_stones: Vec<BigInt> = vec![];
                let stone_as_string = stone.to_string();

                if *stone == BigInt::ZERO {
                    new_stones.push(FromPrimitive::from_usize(1).unwrap());
                } else if stone_as_string.len() % 2 == 0 {
                    let (a, b) = stone_as_string.split_at(stone_as_string.len() / 2);

                    new_stones.push(FromPrimitive::from_usize(a.parse().unwrap()).unwrap());
                    new_stones.push(FromPrimitive::from_usize(b.parse().unwrap()).unwrap());
                } else {
                    new_stones.push(stone * 2024)
                }

                new_stones
            })
            .flatten()
            .collect();

        stones = new_stones;
    }

    stones.len()
}

fn part1(input: String) -> usize {
    get_result(input, 25)
}

fn part2(_input: String) -> usize {
    // get_result(input, 75)

    0
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
