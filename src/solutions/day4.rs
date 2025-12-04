#[allow(unused)]
use crate::{output, OutputLevel, Challenge};
use crate::utils::grid::Grid;

pub struct Day4;

const ADJACENTS: [(isize, isize); 8] = [
    (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)
];

impl Challenge for Day4 {
    fn solve_part1(&self, input: &String) -> Option<i64> {
        let mut result = 0;      

        let grid = parse(input);

        for (row, col) in grid.positions() {
            if let Some(&'@') = grid.get(row as isize, col as isize) {
                let mut adjacent_papers = 0;
                for offset in ADJACENTS {
                    if let Some(&'@') = grid.get(row as isize + offset.0, col as isize + offset.1) {
                        adjacent_papers += 1;
                    }
                }
                if adjacent_papers < 4 {
                    result += 1;
                }
            }
        }

        Some(result)
    }

    #[allow(unused)]
    fn solve_part2(&self, input: &String) -> Option<i64> { 
        let mut result = 0;      

        let mut grid = parse(input);

        loop {
            let mut papers_removed = false;
            for (row, col) in grid.positions() {
                if let Some(&'@') = grid.get(row as isize, col as isize) {
                    let mut adjacent_papers = 0;
                    for offset in ADJACENTS {
                        if let Some(&'@') = grid.get(row as isize + offset.0, col as isize + offset.1) {
                            adjacent_papers += 1;
                        }
                    }
                    if adjacent_papers < 4 {
                        *grid.get_mut(row as isize, col as isize).unwrap() = '.';
                        papers_removed = true;
                        result += 1;
                    }
                }
            }

            if !papers_removed {
                break;
            }
        }

        Some(result)
    }
}

fn parse(input: &String) -> Grid<char> {
    Grid::from(
        input.clone().replace("\n", "").chars().collect(),
        String::from(input.lines().next().unwrap()).len(),
        input.lines().count()
    )
}