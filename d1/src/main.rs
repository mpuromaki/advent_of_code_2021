/*!
# Advent of Code 2021 - Day 01 - Part 1

count the number of times a depth measurement increases from the previous measurement.
In test, there are 7 measurements that are larger than the previous measurement.

## Panics

Panics if input data is not available. Check that .aoc-session file is in your current
working directory. This file should contain your AOC session cookie data.
!*/

static AOC_URL: &'static str = "https://adventofcode.com/2021/day/1/input";

use shared::get_input_aoc;

fn solve(data: &str) -> u32 {
    let mut buffer: Vec<u32> = Vec::new();
    let mut counter: u32 = 0;

    for val in data.lines() {
        let val = val
            .trim()
            .to_owned()
            .parse::<u32>()
            .expect("Could not parse input number.");
        buffer.push(val);
    }

    let mut prev_num = buffer[0];
    for num in buffer[1..].into_iter() {
        if num > &prev_num {
            counter += 1;
        }
        prev_num = *num;
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
        assert_eq!(7, result);
    }
}
