//! <https://adventofcode.com/2021/day/17>

use std::cmp;
use std::ops::RangeInclusive;

const INPUT_TARGET_X_RANGE: RangeInclusive<i32> = 235..=259;
const INPUT_TARGET_Y_RANGE: RangeInclusive<i32> = -118..=-62;

struct Coordinates {
    x: i32,
    y: i32,
}

struct Velocity {
    x: i32,
    y: i32,
}

struct ProbeShotSimulation {
    probe_position: Coordinates,
    probe_velocity: Velocity,

    max_y_coord: i32,

    target_x_range: RangeInclusive<i32>,
    target_y_range: RangeInclusive<i32>,
}

impl ProbeShotSimulation {
    fn is_target_hit(&self) -> bool {
        self.target_x_range.contains(&self.probe_position.x)
            && self.target_y_range.contains(&self.probe_position.y)
    }

    fn is_target_overshot(&self) -> bool {
        let is_probe_right_of_target = self.probe_position.x > *self.target_x_range.end();
        let is_probe_below_target = self.probe_position.y < *self.target_y_range.start();
        is_probe_right_of_target || is_probe_below_target
    }

    fn tick(&mut self) {
        self.probe_position.x += self.probe_velocity.x;
        self.probe_position.y += self.probe_velocity.y;

        self.probe_velocity.x = match self.probe_velocity.x {
            x if x > 0 => x - 1,
            x if x < 0 => x + 1,
            _ => 0,
        };
        self.probe_velocity.y -= 1;

        self.max_y_coord = cmp::max(self.max_y_coord, self.probe_position.y);
    }

    /// Runs this simulation until the target area is either hit (then returns true) or overshot
    /// (then returns false).
    fn run(&mut self) -> bool {
        loop {
            self.tick();

            if self.is_target_hit() {
                break true;
            }
            if self.is_target_overshot() {
                break false;
            }
        }
    }
}

pub fn solution_1() -> String {
    let mut max_y_coord = 0;
    // Initial x velocity < 22 can never reach the target area (https://oeis.org/A000217/list),
    // and we don't want a high x velocity, to give time for the y arc.
    for x in 22..=25 {
        // Suitable initial y velocity range found experimentally :).
        for y in 100..=200 {
            let mut simulation = ProbeShotSimulation {
                probe_position: Coordinates { x: 0, y: 0 },
                probe_velocity: Velocity { x, y },
                max_y_coord: 0,
                target_x_range: INPUT_TARGET_X_RANGE,
                target_y_range: INPUT_TARGET_Y_RANGE,
            };
            if simulation.run() == true {
                max_y_coord = cmp::max(max_y_coord, simulation.max_y_coord);
            }
        }
    }
    max_y_coord.to_string()
}

pub fn solution_2() -> String {
    let mut num_successful_simulations = 0;
    // Initial x velocity < 22 can never reach the target area (https://oeis.org/A000217/list),
    // and > 259 overshoots in the first tick.
    for x in 22..=259 {
        // Initial y velocity < -118 overshoots in the first tick, and the upper bound can be
        // copied from [solution_1].
        for y in -118..=200 {
            let mut simulation = ProbeShotSimulation {
                probe_position: Coordinates { x: 0, y: 0 },
                probe_velocity: Velocity { x, y },
                max_y_coord: 0,
                target_x_range: INPUT_TARGET_X_RANGE,
                target_y_range: INPUT_TARGET_Y_RANGE,
            };
            if simulation.run() == true {
                num_successful_simulations += 1;
            }
        }
    }
    num_successful_simulations.to_string()
}
