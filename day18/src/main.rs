use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

type Coord = (i32, i32);
struct Grid {
    cells: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn at(&self, (x, y): Coord) -> bool {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            false
        } else {
            self.cells[y as usize][x as usize]
        }
    }

    fn at_mut(&mut self, (x, y): Coord) -> &mut bool {
        &mut self.cells[y as usize][x as usize]
    }

    fn neighbours(&self, (x, y): Coord) -> usize {
        let neighbour_coords = [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];

        neighbour_coords
            .into_iter()
            .map(|c| self.at(c))
            .filter(|x| *x)
            .count()
    }

    fn cell_rule(state: bool, neighbours: usize) -> bool {
        matches!((state, neighbours), (true, 2 | 3) | (false, 3))
    }

    fn step(&mut self) {
        let mut next_grid = self.cells.clone();
        for j in 0..self.height {
            for i in 0..self.width {
                next_grid[j][i] = Self::cell_rule(
                    self.at((i as i32, j as i32)),
                    self.neighbours((i as i32, j as i32)),
                );
            }
        }
        self.cells = next_grid;
    }

    fn turn_corners_on(&mut self) {
        *self.at_mut((0, 0)) = true;
        *self.at_mut((self.width as i32 - 1, 0)) = true;
        *self.at_mut((0, self.height as i32 - 1)) = true;
        *self.at_mut((self.width as i32 - 1, self.height as i32 - 1)) = true;
    }

    fn step_corners_on(&mut self) {
        self.turn_corners_on();
        self.step();
        self.turn_corners_on();
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for j in 0..self.height as i32 {
            for i in 0..self.width as i32 {
                if self.at((i, j)) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

fn parse_grid(input: &str) -> Grid {
    let mut cells = vec![];
    let mut width = 0;
    let mut height = 0;
    for (j, line) in input.lines().enumerate() {
        height = j + 1;
        let mut row = vec![];
        for (i, c) in line.chars().enumerate() {
            width = i + 1;
            match c {
                '#' => {
                    row.push(true);
                }
                '.' => {
                    row.push(false);
                }
                _ => panic!(),
            };
        }
        cells.push(row);
    }

    Grid {
        cells,
        width,
        height,
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut grid = parse_grid(input);

    for _ in 0..100 {
        grid.step();
    }

    let num_lights_on = grid.cells.iter().flatten().copied().filter(|x| *x).count();
    println!("{num_lights_on}");

    let mut grid_2 = parse_grid(input);
    for _ in 0..100 {
        grid_2.step_corners_on();
    }
    let num_lights_on_corners_on = grid_2
        .cells
        .iter()
        .flatten()
        .copied()
        .filter(|x| *x)
        .count();
    println!("{num_lights_on_corners_on}");
}

#[test]
fn test() {
    let input = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..\n";
    let mut grid = parse_grid(input);

    for _ in 0..4 {
        println!("{}", &grid);
        grid.step();
    }
    println!("{}", &grid);
}
