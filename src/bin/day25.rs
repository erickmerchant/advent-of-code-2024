type Item = (char, [usize; 5]);

fn get_answer(input: Vec<String>) -> usize {
    let mut items: Vec<Item> = Vec::new();
    let mut result = 0;
    let mut current: Option<Item> = None;
    let mut current_lines = 0;

    for line in input {
        if line.is_empty() {
            continue;
        }

        if current_lines == 5 {
            if let Some((key1, val1)) = current {
                for (key2, val2) in items.clone() {
                    if key1 != key2 {
                        let mut is_result = true;

                        for i in 0..5 {
                            if val1[i] + val2[i] > 5 {
                                is_result = false;

                                break;
                            }
                        }

                        if is_result {
                            result += 1;
                        }
                    }
                }

                items.push(current.unwrap());
            }

            current = None;
            current_lines = 0;

            continue;
        }

        let line = line.chars().collect::<Vec<char>>();

        if let Some((key, val)) = current {
            let mut val = val;

            for i in 0..5 {
                if line[i] == '#' {
                    val[i] += 1;
                }
            }

            current = Some((key, val));

            current_lines += 1;
        } else {
            current = Some((line[0], [0, 0, 0, 0, 0]));
        }
    }

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
        "#####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_get_answer() {
        let fixture = get_fixture();

        assert_eq!(get_answer(fixture), 3);
    }
}
