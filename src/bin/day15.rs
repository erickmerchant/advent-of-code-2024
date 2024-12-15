fn get_answer(input: Vec<String>) -> usize {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut grid_done = false;
    let mut directions: Vec<char> = vec![];
    let mut position = (0, 0);

    for (y, line) in input.iter().enumerate() {
        if line.is_empty() {
            grid_done = true;
            continue;
        }

        let mut chars: Vec<char> = line.chars().collect();

        if let Some(x) = chars.iter().position(|c| *c == '@') {
            position = (x, y);
        }

        if !grid_done {
            grid.push(chars);
        } else {
            directions.append(&mut chars);
        }
    }

    for direction in directions {
        match direction {
            '^' => {}
            'v' => {}
            '<' => {}
            '>' => {}
            _ => {}
        }
    }

    // println!("{:?}", grid);
    // println!("{:?}", directions);

    0
}

fn main() {
    let input = advent::get_input();

    println!("{}", get_answer(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture1() -> Vec<String> {
        "########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########

        <^^>>>vv<v>>v<<"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    fn get_fixture2() -> Vec<String> {
        "##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test1_get_answer() {
        let fixture = get_fixture1();

        assert_eq!(get_answer(fixture), 2028);
    }

    #[test]
    fn test2_get_answer() {
        let fixture = get_fixture2();

        assert_eq!(get_answer(fixture), 10092);
    }
}
