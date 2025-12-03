use std::{fs::File, io::Read, process::exit};

use advent_of_code_25::*;


fn main() {
    let matches = crate::command_parser().get_matches();

    let day: u8 = matches.get_one::<String>("DAY").unwrap().parse().unwrap();
    if day as usize > CHALLENGES.len() || day <= 0{
        eprintln!("Day {day} isn't implemented!");
        exit(1);
    }

    let disable = match matches.get_many::<String>("disable") {
        Some(values) => values.collect::<Vec<_>>(),
        None => Vec::new()
    };

    let challenge = &CHALLENGES[day as usize - 1];

    let tests = [
        Test::new("Example 1", "examples/day{}.txt", Box::new(|input| challenge.solve_part1(input))),
        Test::new("Puzzle 1", "puzzles/day{}.txt", Box::new(|input| challenge.solve_part1(input))),
        Test::new("Example 2", "examples/day{}.txt", Box::new(|input| challenge.solve_part2(input))),
        Test::new("Puzzle 2", "puzzles/day{}.txt", Box::new(|input| challenge.solve_part2(input))),
    ];

    for test in tests {
        if disable.contains(&&String::from(test.name)) {
            println!("Skipping {}... (from command line args)", test.name);
            continue;
        }

        let path = String::from("challenge_files/") + &test.file.replace("{}", &day.to_string());
        let mut contents = String::new();
        match File::open(&path) {
            Ok(mut file) => {
                match file.read_to_string(&mut contents) {
                    Ok(_) => (),
                    Err(error) => println!("Skipping {}... (error while reading file: \"{}\"", test.name, error)
                }
            },
            Err(_) => {
                println!("Skipping {}... (unable to find \"{}\")", test.name, path);
                continue;
            }
        }

        match (test.callback)(&contents) {
            Some(result) => println!("{}: {}", test.name, result),
            None => println!("{} isn't implemented yet!", test.name)
        }
    }
}
