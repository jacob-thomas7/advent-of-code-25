pub struct Day1;

impl super::Challenge for Day1 {
    fn solve_part1(&self, input: &mut String) -> i64 {
        let mut result = 0;
        let mut current_rotation: i16 = 50;
        for line in input.lines() {
            let direction = &line[0..1];
            let count = &line[1..].parse::<i16>().unwrap();

            match direction {
                "R" => current_rotation = (current_rotation + count) % 100,
                "L" => current_rotation = (current_rotation - count) % 100,
                _ => panic!("Invalid direction {direction}")
            }

            if current_rotation == 0 {
                result += 1;
            }
        }
        result
    }

    fn solve_part2(&self, input: &mut String) -> i64 {
        let mut result = 0;
        let mut current_rotation: i16 = 50;
        for line in input.lines() {
            let direction = &line[0..1];
            let count = &line[1..].parse::<i16>().unwrap();

            let previous = current_rotation;

            match direction {
                "R" => current_rotation = current_rotation + count,
                "L" => current_rotation = current_rotation - count,
                _ => panic!("Invalid direction {direction}")
            }

            if (previous < 0 && current_rotation > 0) || (previous >= 0 && current_rotation < 0) {
                println!("Add 0 from {previous} to {current_rotation}");
                result += 1;
            } else {
                println!("from {previous} to {current_rotation}")
            }

            while current_rotation < -100 {
                current_rotation += 100;
                println!("There was a rotation");
                result += 1;
            }
            
            while current_rotation > 100 {
                current_rotation -= 100;
                println!("There was a rotation");
                result += 1;
            }
            
            if previous == 0 && current_rotation > 0 {
                println!("There was a rotation");
                result += 1;
            }

            while current_rotation < 0 {
                current_rotation += 100;
            }


            if current_rotation == 100 {
                current_rotation = 0;
            }
        }
        result
    }
}