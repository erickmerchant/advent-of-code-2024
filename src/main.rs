pub mod day01;
pub mod day02;

use clap::Parser;
use core::panic;
use std::io::{self, BufRead};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, short)]
    day: u8,
    #[arg(long, short)]
    part: u8,
}

fn main() {
    let args = Args::parse();
    let stdin = io::stdin();
    let handle = stdin.lock();
    let input = handle.lines().map_while(Result::ok).collect();
    let output = match (args.day, args.part) {
        (1, 1) => day01::part1(input),
        (1, 2) => day01::part2(input),
        (2, 1) => day02::part1(input),
        (2, 2) => day02::part2(input),
        _ => panic!("Incomplete day or part"),
    };

    println!("{output}");
}
