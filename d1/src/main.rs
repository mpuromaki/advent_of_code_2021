/*!
# Advent of Code 2021 - Day 01 - Part 2

Consider sums of a three-measurement sliding window. Start by comparing the first and second
three-measurement windows. The measurements in the first window are marked A (199, 200, 208);
their sum is 199 + 200 + 208 = 607. The second window is marked B (200, 208, 210); its sum is
618. The sum of measurements in the second window is larger than the sum of the first, so this
first comparison increased.

Your goal now is to count the number of times the sum of measurements in this sliding window
increases from the previous sum. So, compare A with B, then compare B with C, then C with D,
and so on. Stop when there aren't enough measurements left to create a new three-measurement sum.

## Panics

Panics if input data is not available. Check that .aoc-session file is in your current
working directory. This file should contain your AOC session cookie data.
!*/

static AOC_URL: &'static str = "https://adventofcode.com/2021/day/1/input";

use shared::get_input_aoc;

fn solve(data: &str) -> u32 {
    let mut buffer: Vec<u32> = Vec::new();
    let mut window: Vec<u32> = Vec::with_capacity(3);
    let mut counter: u32 = 0;

    for val in data.lines() {
        let val = val
            .trim()
            .to_owned()
            .parse::<u32>()
            .expect("Could not parse input number.");
        buffer.push(val);
    }

    window.insert(0, buffer[0]);
    window.insert(0, buffer[1]);
    window.insert(0, buffer[2]);

    let mut prev_num: u32 = window.iter().sum();
    for num in buffer[3..].into_iter() {
        // step the window
        window.truncate(2);
        window.insert(0, *num);

        // compare values
        let sum = window.iter().sum();
        if sum > prev_num {
            counter += 1;
        }
        prev_num = sum;
    }

    return counter;
}

fn main() {
    println!("Advent of Code 2021 - Day 01");
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
            "199
            200
            208
            210
            200
            207
            240
            269
            260
            263",
        );

        let result = solve(&data);
        assert_eq!(5, result);
    }
}
