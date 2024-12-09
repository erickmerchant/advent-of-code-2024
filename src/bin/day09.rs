fn part1(input: String) -> usize {
    let input = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    let mut result: Vec<Option<usize>> = vec![];
    let mut is_on = true;
    let mut to_move_count = 0;
    let mut id = 0;

    for d in input {
        if is_on {
            for _ in 0..d {
                result.push(Some(id));
            }

            id += 1;
        } else {
            for _ in 0..d {
                result.push(None);
            }

            to_move_count += d;
        }

        is_on = !is_on;
    }

    loop {
        if to_move_count == 0 {
            break;
        }

        if let (Some(a), Some(b)) = (
            result.iter().position(|i| i.is_none()),
            result.iter().rposition(|i| i.is_some()),
        ) {
            result.swap(a, b)
        }

        to_move_count -= 1;
    }

    let mut total = 0;

    for (i, d) in result.iter().enumerate() {
        if let Some(d) = d {
            total += d * i;
        }
    }

    total
}

fn part2(input: String) -> usize {
    let input = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    let mut result: Vec<Option<usize>> = vec![];
    let mut is_on = true;
    let mut to_move_count = 0;
    let mut id = 0;

    for d in input {
        if is_on {
            for _ in 0..d {
                result.push(Some(id));
            }

            id += 1;
        } else {
            for _ in 0..d {
                result.push(None);
            }

            to_move_count += d;
        }

        is_on = !is_on;
    }

    loop {
        if to_move_count == 0 {
            break;
        }

        if let (Some(a), Some(b)) = (
            result.iter().position(|i| i.is_none()),
            result.iter().rposition(|i| i.is_some()),
        ) {
            result.swap(a, b)
        }

        to_move_count -= 1;
    }

    let mut total = 0;

    for (i, d) in result.iter().enumerate() {
        if let Some(d) = d {
            total += d * i;
        }
    }

    total
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
        "2333133121414131402".to_string()
    }

    #[test]
    fn test_part1() {
        let fixture = get_fixture();

        assert_eq!(part1(fixture), 1928);
    }

    #[test]
    fn test_part2() {
        let fixture = get_fixture();

        assert_eq!(part2(fixture), 2858);
    }
}
