use rayon::prelude::*;

fn get_answer(input: Vec<String>) -> usize {
    let parts: Vec<String> = input[0].split(", ").map(|s| s.to_string()).collect();
    let patterns: Vec<String> = input[2..].iter().map(|s| s.to_string()).collect();

    let result = patterns
        .par_iter()
        .map(|pattern| {
            let parts: Vec<String> = parts
                .iter()
                .filter(|&part| pattern.contains(part))
                .cloned()
                .collect();
            let mut possibles: Vec<String> = parts
                .iter()
                .filter(|&part| pattern.starts_with(part))
                .map(|part| pattern.trim_start_matches(part).to_string())
                .collect();

            loop {
                let mut new_possibles = vec![];

                for possible in &possibles {
                    for part in &parts {
                        if possible.starts_with(part) {
                            let new_possible = possible.trim_start_matches(part).to_string();

                            if new_possible.is_empty() {
                                return 1;
                            }

                            if !new_possibles.contains(&new_possible) {
                                new_possibles.push(new_possible);
                            }
                        }
                    }
                }

                if new_possibles.is_empty() {
                    return 0;
                }

                possibles = new_possibles;
            }
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
        "r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_get_answer() {
        let fixture = get_fixture();

        assert_eq!(get_answer(fixture), 6);
    }
}
