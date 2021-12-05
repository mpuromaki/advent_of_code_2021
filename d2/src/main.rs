/*!
# Advent of Code 2021 - Day 02 - Part 2

In addition to horizontal position and depth, you'll also need to track a third value, aim,
which also starts at 0.

* down X increases your aim by X units.
* up X decreases your aim by X units.
* forward X does two things:
  * It increases your horizontal position by X units.
  * It increases your depth by your aim multiplied by X.


## Panics

Panics if input data is not available. Check that .aoc-session file is in your current
working directory. This file should contain your AOC session cookie data.
!*/

static AOC_URL: &'static str = "https://adventofcode.com/2021/day/2/input";

use shared::get_input_aoc;

enum Direction {
    Forward,
    Up,
    Down,
}

struct Step {
    dir: Direction,
    val: u32,
}

struct Submarine {
    position: u32,
    depth: u32,
    aim: u32,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            position: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn step(&mut self, step: Step) {
        match step {
            Step {
                dir: Direction::Forward,
                val,
            } => {
                self.position += val;
                self.depth += val * self.aim;
            }
            Step {
                dir: Direction::Up,
                val,
            } => self.aim -= val,
            Step {
                dir: Direction::Down,
                val,
            } => self.aim += val,
        }
    }

    fn result(&self) -> u32 {
        return self.position * self.depth;
    }
}

fn solve(data: &str) -> u32 {
    let mut buffer: Vec<Step> = Vec::new();

    // Parse input data into list of tuples (direction, value).
    for val in data.lines() {
        let line = val.trim().to_owned();
        let tuple = line.split_once(" ").expect("Split failed.");

        let dir = match tuple.0 {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            y => panic!("Could not parse direction: {}", y),
        };
        let val = tuple.1.parse::<u32>().expect("Could not parse value.");
        buffer.push(Step { dir, val });
    }

    let mut sub = Submarine::new();
    for step in buffer {
        sub.step(step);
    }
    return sub.result();
}

fn main() {
    println!("Advent of Code 2021 - Day 02");
    let data = get_input_aoc(AOC_URL);

    let result = match data {
        Ok(data) => solve(&data),
        Err(e) => panic!("{}", e),
    };

    println!("Result is: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testdata() {
        let data = String::from(
            "forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2",
        );

        let result = solve(&data);
        assert_eq!(900, result);
    }
}
