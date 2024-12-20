fn part1(input: String) -> usize {
    let input = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    let mut result: Vec<Option<usize>> = Vec::new();
    let mut is_on = true;
    let mut to_move_count = 0;
    let mut id = 0;

    for count in input {
        let new_result;

        if is_on {
            new_result = Some(id);

            id += 1;
        } else {
            new_result = None;

            to_move_count += count;
        }

        result.append(&mut vec![new_result; count]);

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

#[derive(Debug, Clone)]
enum Block {
    Used(usize),
    Free(usize),
}

struct Span {
    pub id: usize,
    pub count: usize,
}

fn part2(input: String) -> usize {
    let input = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    let mut result: Vec<Block> = Vec::new();
    let mut is_on = true;
    let mut id = 0;
    let mut free_map: Vec<Span> = Vec::new();
    let mut used_map: Vec<Span> = Vec::new();

    for count in input {
        let new_result;

        if is_on {
            new_result = Block::Used(id);

            used_map.push(Span { id, count });

            id += 1;
        } else {
            new_result = Block::Free(id);

            free_map.push(Span { id, count });
        }

        result.append(&mut vec![new_result; count]);

        is_on = !is_on;
    }

    for span in used_map.iter().rev() {
        if let Some(index_of_free) = free_map.iter().position(|s| s.count >= span.count) {
            let free = free_map.get_mut(index_of_free).unwrap();
            let mut to_move_count = span.count;

            free.count -= span.count;

            loop {
                if to_move_count == 0 {
                    break;
                }

                if let (Some(a), Some(b)) = (
                    result.iter().position(|r| {
                        if let Block::Free(i) = r {
                            *i == free.id
                        } else {
                            false
                        }
                    }),
                    result.iter().rposition(|r| {
                        if let Block::Used(i) = r {
                            *i == span.id
                        } else {
                            false
                        }
                    }),
                ) {
                    if a < b {
                        result.swap(a, b);

                        to_move_count -= 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let mut total = 0;

    for (i, d) in result.iter().enumerate() {
        if let Block::Used(d) = d {
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
