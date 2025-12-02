use clap::{Command, arg};

pub mod day1;
pub mod day2;

pub trait Challenge {
    fn solve_part1(&self, input: &mut String) -> i64;
    fn solve_part2(&self, input: &mut String) -> i64;
}

pub const CHALLENGES: [&dyn Challenge; 2] = [
    &day1::Day1{},
    &day2::Day2{},
];

pub fn command_parser() -> Command {
    Command::new("Advent of Code 2025")
        .arg(arg!(<DAY>))
        .arg(arg!(-d --disable [OPTIONS]))
}