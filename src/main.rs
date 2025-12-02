use std::{fs::File, io::Read, process::exit};

use advent_of_code_25::*;

fn main() {
    let matches = crate::command_parser().get_matches();

    let day: u8 = matches.get_one::<String>("DAY").unwrap().parse().unwrap();
    if day as usize > CHALLENGES.len()  || day <= 0{
        eprintln!("Day {day} isn't implemented!");
        exit(1);
    }
    
    let challenge = &CHALLENGES[day as usize - 1];

    let mut example1 = String::new();
    let mut puzzle1 = String::new();
    let mut example2 = String::new();
    let mut puzzle2 = String::new();

    File::open(format!("challenge_files/examples1/day{day}.txt")).unwrap().read_to_string(&mut example1).unwrap();
    File::open(format!("challenge_files/puzzles/day{day}.txt")).unwrap().read_to_string(&mut puzzle1).unwrap();
    File::open(format!("challenge_files/examples1/day{day}.txt")).unwrap().read_to_string(&mut example2).unwrap();
    File::open(format!("challenge_files/puzzles/day{day}.txt")).unwrap().read_to_string(&mut puzzle2).unwrap();

    println!("Example 1: {}", challenge.solve_part1(&mut example1));
    println!("Part 1: {}", challenge.solve_part1(&mut puzzle1));
    println!("Example 2: {}", challenge.solve_part2(&mut example2));
    println!("Part 2: {}", challenge.solve_part2(&mut puzzle2));
}
