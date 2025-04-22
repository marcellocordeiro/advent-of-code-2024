use regex::Regex;

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE_1: &str = include_str!("inputs/sample_1.txt");
pub const SAMPLE_2: &str = include_str!("inputs/sample_2.txt");

pub mod part1;
pub mod part2;

enum Pattern {
    Mul { a: i32, b: i32 },
    Do,
    Dont,
}

fn parse_input(input: &str) -> Vec<Pattern> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut matches = vec![];

    for m in re.captures_iter(input) {
        let instruction = m.get(0).unwrap().as_str();

        match instruction {
            "do()" => matches.push(Pattern::Do),
            "don't()" => matches.push(Pattern::Dont),
            _ => {
                let a = m.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let b = m.get(2).unwrap().as_str().parse::<i32>().unwrap();

                matches.push(Pattern::Mul { a, b });
            }
        }
    }

    matches
}
