use rayon::prelude::*;

fn get_answer(input: Vec<String>) -> usize {
    let result = input
        .par_iter()
        .map(|line| line.parse::<usize>().unwrap())
        .map(|input| {
            let mut secret = input;

            for _ in 0..2000 {
                let result = secret * 64;

                secret ^= result;
                secret %= 16777216;

                let result = secret / 32;

                secret ^= result;
                secret %= 16777216;

                let result = secret * 2048;

                secret ^= result;
                secret %= 16777216;
            }

            secret
        })
        .sum();

    result
}

fn main() {
    let input = advent::get_input();

    println!("{}", get_answer(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        "1
        10
        100
        2024"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_get_answer() {
        let fixture = get_fixture();

        assert_eq!(get_answer(fixture), 37327623);
    }
}
