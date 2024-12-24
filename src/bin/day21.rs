use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
enum Button {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Activate,
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Button {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::Activate,
            '>' => Self::Right,
            '<' => Self::Left,
            '^' => Self::Up,
            '0' => Self::Zero,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            _ => panic!(),
        }
    }
}

trait Keypad {
    fn get_coords(button: Button) -> (isize, isize);

    fn get_valid_coords() -> Vec<(isize, isize)>;

    fn get_directions(buttons: Vec<Button>, depth: usize) -> Vec<Button> {
        let mut results: Vec<Vec<Button>> = Vec::new();
        let mut previous = Button::Activate;

        for button in buttons {
            let (x1, y1) = Self::get_coords(previous);
            let (x2, y2) = Self::get_coords(button);
            let x = x2 - x1;
            let y = y2 - y1;
            let mut res = Vec::new();

            match y.cmp(&0) {
                Ordering::Greater => res.append(&mut [Button::Down].repeat(y.unsigned_abs())),
                Ordering::Less => res.append(&mut [Button::Up].repeat(y.unsigned_abs())),
                Ordering::Equal => (),
            }

            match x.cmp(&0) {
                Ordering::Greater => res.append(&mut [Button::Right].repeat(x.unsigned_abs())),
                Ordering::Less => res.append(&mut [Button::Left].repeat(x.unsigned_abs())),
                Ordering::Equal => (),
            }

            let mut res = res
                .iter()
                .permutations(res.clone().len())
                .unique()
                .map(|r| {
                    let mut r = r.to_vec();
                    r.push(&Button::Activate);
                    r
                })
                .collect::<Vec<_>>();

            let mut new_results: Vec<Vec<Button>> = Vec::new();

            for r in res.iter_mut() {
                let valid = if r.is_empty() {
                    true
                } else {
                    let (mut x1, mut y1) = Self::get_coords(previous);
                    let mut valid = true;

                    for rr in r.clone() {
                        let (x2, y2) = match rr {
                            Button::Up => (x1, y1 - 1),
                            Button::Down => (x1, y1 + 1),
                            Button::Left => (x1 - 1, y1),
                            Button::Right => (x1 + 1, y1),
                            Button::Activate => (x1, y1),
                            _ => panic!(),
                        };
                        if !Self::get_valid_coords().contains(&(x2, y2)) {
                            valid = false;
                            break;
                        }
                        x1 = x2;
                        y1 = y2;
                    }

                    valid
                };

                if valid {
                    let r = r.iter_mut().map(|x| x.to_owned()).collect::<Vec<_>>();

                    if results.is_empty() {
                        new_results.push(r);
                    } else {
                        for result in results.iter_mut() {
                            let mut result = result.clone();
                            result.append(&mut r.clone());
                            new_results.push(result);
                        }
                    }
                }
            }

            previous = button;

            results = new_results;
        }

        let mut shortest: Option<Vec<Button>> = None;

        for result in results {
            let mut result = result;

            if depth != 0 {
                result = DirectionalKeypad::get_directions(result, depth - 1);
            }

            if shortest.is_none() || result.len() < shortest.clone().unwrap().len() {
                shortest = Some(result.clone());
            }
        }

        shortest.unwrap()
    }
}

struct DirectionalKeypad;

impl Keypad for DirectionalKeypad {
    fn get_coords(button: Button) -> (isize, isize) {
        match button {
            Button::Up => (1, 0),
            Button::Activate => (2, 0),
            Button::Left => (0, 1),
            Button::Down => (1, 1),
            Button::Right => (2, 1),
            _ => panic!(),
        }
    }

    fn get_valid_coords() -> Vec<(isize, isize)> {
        vec![(1, 0), (2, 0), (0, 1), (1, 1), (2, 1)]
    }
}

struct NumericKeypad;

impl Keypad for NumericKeypad {
    fn get_coords(button: Button) -> (isize, isize) {
        match button {
            Button::Seven => (0, 0),
            Button::Eight => (1, 0),
            Button::Nine => (2, 0),
            Button::Four => (0, 1),
            Button::Five => (1, 1),
            Button::Six => (2, 1),
            Button::One => (0, 2),
            Button::Two => (1, 2),
            Button::Three => (2, 2),
            Button::Zero => (1, 3),
            Button::Activate => (2, 3),
            _ => panic!(),
        }
    }

    fn get_valid_coords() -> Vec<(isize, isize)> {
        vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (1, 3),
            (2, 3),
        ]
    }
}

#[derive(Debug, Clone)]
struct Code {
    steps: Vec<Button>,
    value: usize,
}

fn get_answer(input: Vec<String>) -> usize {
    let mut codes: Vec<Code> = Vec::new();

    for line in input {
        let mut len = line.len() - 2;
        let mut steps: Vec<Button> = Vec::new();
        let mut value: usize = 0;

        for c in line.chars() {
            let step = Button::from(c);

            if step != Button::Activate {
                value += c.to_string().parse::<usize>().unwrap() * 10_usize.pow(len as u32);

                len = len.saturating_sub(1);
            }

            steps.push(step);
        }

        codes.push(Code { steps, value });
    }

    let mut result = 0;

    for code in codes {
        let directions = NumericKeypad::get_directions(code.clone().steps, 2);

        result += directions.len() * code.value;
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
        "029A
        980A
        179A
        456A
        379A"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_get_answer() {
        let fixture = get_fixture();

        assert_eq!(get_answer(fixture), 126384);
    }
}
