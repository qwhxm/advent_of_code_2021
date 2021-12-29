//! <https://adventofcode.com/2021/day/11>

use grid::{grid, Grid};

const INPUT: [&str; 10] = [
    "1172728874",
    "6751454281",
    "2612343533",
    "1884877511",
    "7574346247",
    "2117413745",
    "7766736517",
    "4331783444",
    "4841215828",
    "6857766273",
];

struct OctopusGrid(Grid<u8>);

/// Grid coordinates (row, column).
type Coordinates = (i32, i32);

impl OctopusGrid {
    fn from_rows(rows: &[&str]) -> OctopusGrid {
        let mut grid: Grid<u8> = grid![];
        for &row in rows {
            grid.push_row(row.chars().map(|c| c.to_digit(10).unwrap() as u8).collect());
        }
        OctopusGrid(grid)
    }

    fn adjacent_coordinates(&self, coords: Coordinates) -> Vec<Coordinates> {
        [
            (coords.0 - 1, coords.1 - 1),
            (coords.0 - 1, coords.1),
            (coords.0 - 1, coords.1 + 1),
            (coords.0, coords.1 - 1),
            (coords.0, coords.1 + 1),
            (coords.0 + 1, coords.1 - 1),
            (coords.0 + 1, coords.1),
            (coords.0 + 1, coords.1 + 1),
        ]
        .into_iter()
        .filter(|&(r, c)| self.0.get(r as usize, c as usize) != None)
        .collect()
    }

    /// Simulates one step of the octopus flashing process, and returns the number of flashes
    /// that occurred.
    fn tick(&mut self) -> u32 {
        for r in 0..self.0.rows() {
            for c in 0..self.0.cols() {
                self.0[r][c] += 1;
            }
        }

        let mut num_flashes = 0;
        loop {
            let mut num_flashes_this_iteration = 0;
            for r in 0..self.0.rows() {
                for c in 0..self.0.cols() {
                    if self.0[r][c] > 9 {
                        num_flashes_this_iteration += 1;
                        self.0[r][c] = 0;

                        for (ar, ac) in self
                            .adjacent_coordinates((r as i32, c as i32))
                            .into_iter()
                            .map(|(r, c)| (r as usize, c as usize))
                        {
                            // Only increase energy level of adjacent non-flashed octopuses -
                            // octopuses that flashed stay at 0.
                            if self.0[ar][ac] != 0 {
                                self.0[ar][ac] += 1;
                            }
                        }
                    }
                }
            }
            num_flashes += num_flashes_this_iteration;

            if num_flashes_this_iteration == 0 {
                break num_flashes;
            }
        }
    }
}

pub fn solution_1() -> String {
    let mut grid = OctopusGrid::from_rows(&INPUT);

    let mut num_flashes = 0;
    for _ in 0..100 {
        num_flashes += grid.tick();
    }
    num_flashes.to_string()
}

pub fn solution_2() -> String {
    let mut grid = OctopusGrid::from_rows(&INPUT);
    let num_octopuses = (grid.0.size().0 * grid.0.size().1) as u32;

    let mut num_steps = 0;
    loop {
        let num_flashes = grid.tick();
        num_steps += 1;

        if num_flashes == num_octopuses {
            break num_steps.to_string();
        }
    }
}
