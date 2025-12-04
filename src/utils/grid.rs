pub struct Grid<T> {
    contents: Vec<T>,
    width: usize,
    height: usize,
}

pub struct GridPositions {
    max: usize,
    height: usize,
    current: usize,
}

impl<T> Grid<T> {
    pub fn from(contents: Vec<T>, width: usize, height: usize) -> Self {
        assert_eq!(contents.len(), width * height);
        Self { contents, width, height }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        if x >= self.width as isize || y >= self.height as isize || x < 0 || y < 0{
            None
        } else {
            *&self.contents.get(x as usize * self.width + y as usize)
        }
    }

    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        if x >= self.width as isize || y >= self.height as isize || x < 0 || y < 0{
            None
        } else {
            self.contents.get_mut(x as usize * self.width + y as usize)
        }
    }

    pub fn positions(&self) -> GridPositions {
        GridPositions {
            max: self.contents.len(),
            height: self.height,
            current: 0
        }
    }
}

impl Iterator for GridPositions {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        if self.current >= self.max {
            None
        } else {
            let x = self.current % self.height;
            let y = self.current / self.height;
            self.current += 1;
            Some((x, y))
        }
    }
} 

impl<T : ToString> std::fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::with_capacity((self.width + 1) * self.height);
        for row in 0..self.height {
            for col in 0..self.width {
                result.push_str(&self.contents.get(row * self.width + col).unwrap().to_string());
            }
            result.push('\n');
        }
        write!(f, "{result}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn grid_display() {
        let grid = String::from("010101010");
        let grid = Grid::from(grid.chars().collect(), 3, 3);
        println!("{grid}");
    }

    #[test]
    fn grid_get() {
        let grid = String::from("123456789");
        let grid = Grid::from(grid.chars().collect(), 3, 3);

        assert_eq!(grid.get(0, 0), Some(&'1'));
        assert_eq!(grid.get(1, 1), Some(&'5'));
        assert_eq!(grid.get(2, 2), Some(&'9'));
        assert_eq!(grid.get(3, 3), None);
    }

    #[test]
    fn grid_positions() {
        let grid = String::from("1234");
        let grid = Grid::from(grid.chars().collect(), 2, 2);
        let mut iter = grid.positions();

        assert_eq!(iter.next(), Some((0, 0)));
        assert_eq!(iter.next(), Some((1, 0)));
        assert_eq!(iter.next(), Some((0, 1)));
        assert_eq!(iter.next(), Some((1, 1)));
        assert_eq!(iter.next(), None);
    }
}