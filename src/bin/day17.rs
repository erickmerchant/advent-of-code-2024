use regex::Regex;
use std::sync::LazyLock;

#[derive(Debug)]
struct Device {
    registers: [usize; 3],
    program: Vec<usize>,
    instruction_pointer: usize,
    output: Vec<usize>,
}

impl Device {
    fn run(&mut self) {
        while self.instruction_pointer + 1 < self.program.len() {
            let operator = self.program[self.instruction_pointer];
            let literal_operand = self.program[self.instruction_pointer + 1];
            let combo_operand = match literal_operand {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => self.registers[0],
                5 => self.registers[1],
                6 => self.registers[2],
                _ => panic!("Invalid operand"),
            };

            match operator {
                0 => {
                    self.registers[0] /= 2_usize.pow(combo_operand as u32);
                    self.instruction_pointer += 2;
                }
                1 => {
                    self.registers[1] = self.registers[1] ^ literal_operand;
                    self.instruction_pointer += 2;
                }
                2 => {
                    self.registers[1] = combo_operand % 8;
                    self.instruction_pointer += 2;
                }
                3 => {
                    if self.registers[0] == 0 {
                        self.instruction_pointer += 2;
                    } else {
                        self.instruction_pointer = literal_operand;
                    }
                }
                4 => {
                    self.registers[1] ^= self.registers[2];
                    self.instruction_pointer += 2;
                }
                5 => {
                    self.output.push(combo_operand % 8);
                    self.instruction_pointer += 2;
                }
                6 => {
                    self.registers[1] = self.registers[0] / 2_usize.pow(combo_operand as u32);
                    self.instruction_pointer += 2;
                }
                7 => {
                    self.registers[2] = self.registers[0] / 2_usize.pow(combo_operand as u32);
                    self.instruction_pointer += 2;
                }
                _ => {}
            }
        }
    }

    fn new(input: String) -> Self {
        static NUM_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(-?\d+)").unwrap());

        let nums: Vec<usize> = NUM_REGEX
            .find_iter(input.as_str())
            .map(|m| m.as_str().parse::<usize>().unwrap())
            .collect();

        let device = Device {
            registers: nums[0..3].try_into().unwrap(),
            program: nums[3..].to_vec(),
            instruction_pointer: 0,
            output: vec![],
        };

        device
    }
}

fn part1(input: String) -> String {
    let mut device = Device::new(input);

    device.run();

    device
        .output
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

// fn part2(input: String) -> usize {
//     let mut device = Device::new(input);

//     loop {
//         device.run();

//         device.registers[0] += 1;

//         if device.output == device.program {
//             break;
//         }
//     }

//     device.registers[0]
// }

fn main() {
    let input = advent::get_input().join("\n");

    println!("{}", part1(input.clone()));
    // println!("{}", part2(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture1() -> String {
        "Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    // fn get_fixture2() -> String {
    //     "Register A: 2024
    //     Register B: 0
    //     Register C: 0

    //     Program: 0,3,5,4,3,0"
    //         .split('\n')
    //         .map(|s| s.trim().to_string())
    //         .collect::<Vec<_>>()
    //         .join("\n")
    // }

    #[test]
    fn test_part1() {
        let fixture = get_fixture1();

        assert_eq!(part1(fixture), "4,6,3,5,6,3,5,2,1,0");
    }

    // #[test]
    // fn test_part2() {
    //     let fixture = get_fixture2();

    //     assert_eq!(part2(fixture), 117440);
    // }
}
