use std::collections::{HashMap, HashSet};

fn get_answer(input: Vec<String>, min_time: usize) -> usize {
    let mut steps: HashMap<(usize, usize), Option<usize>> = HashMap::new();
    let mut walls: HashSet<(usize, usize)> = HashSet::new();
    let mut current = (0, 0);
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    current = (x, y);
                    start = (x, y);

                    steps.insert((x, y), Some(0));
                }
                '.' => {
                    steps.insert((x, y), None);
                }
                'E' => {
                    end = (x, y);

                    steps.insert((x, y), None);
                }
                _ => {}
            }
        }
    }

    loop {
        let (x, y) = current;
        let current_time = steps.get(&(x, y)).unwrap().unwrap();
        let mut next = None;
        let mut directions = Vec::new();

        if (x + 1, y) != start {
            directions.push((x + 1, y));
        }

        if (x, y + 1) != start {
            directions.push((x, y + 1));
        }

        if (x - 1, y) != start {
            directions.push((x - 1, y));
        }

        if (x, y - 1) != start {
            directions.push((x, y - 1));
        }

        for (x, y) in directions {
            if steps.get(&(x, y)).map(|t| t.is_none()).unwrap_or(false) {
                steps.insert((x, y), Some(current_time + 1));

                next = Some((x, y));
            } else if !steps.contains_key(&(x, y)) {
                walls.insert((x, y));
            }
        }

        if let Some(next) = next {
            if next == current || next == end {
                break;
            }

            current = next;
        } else {
            break;
        }
    }

    let mut results = Vec::new();

    for (x, y) in walls {
        let north = if y > 0 { steps.get(&(x, y - 1)) } else { None };
        let south = steps.get(&(x, y + 1));
        let east = steps.get(&(x + 1, y));
        let west = if x > 0 { steps.get(&(x - 1, y)) } else { None };

        if let (Some(n), Some(s)) = (north, south) {
            if n.is_some() && s.is_some() {
                results.push((n.unwrap() as isize - s.unwrap() as isize).unsigned_abs() - 2);
            }
        }

        if let (Some(e), Some(w)) = (east, west) {
            if e.is_some() && w.is_some() {
                results.push((e.unwrap() as isize - w.unwrap() as isize).unsigned_abs() - 2);
            }
        }
    }

    results.iter().filter(|r| *r >= &min_time).count()
}

fn main() {
    let input = advent::get_input();

    println!("{}", get_answer(input.clone(), 100));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        "###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_get_answer() {
        let fixture = get_fixture();

        assert_eq!(get_answer(fixture, 0), 44);
    }
}
