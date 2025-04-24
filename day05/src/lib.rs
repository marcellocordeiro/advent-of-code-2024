pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub mod part1;
pub mod part2;

#[derive(Debug, PartialEq, Eq)]
struct OrderingRule {
    x: i32,
    y: i32,
}

type PageNumbers = Vec<i32>;

#[derive(Debug)]
struct Input {
    ordering_rules: Vec<OrderingRule>,
    updates: Vec<PageNumbers>,
}

fn parse_input(input: &str) -> Input {
    let (ordering_rules, updates) = input.trim().split_once("\n\n").unwrap();

    let ordering_rules = ordering_rules
        .lines()
        .map(|line| {
            let (x, y) = line
                .split_once('|')
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap();

            OrderingRule { x, y }
        })
        .collect();

    let updates = updates
        .lines()
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();

    Input {
        ordering_rules,
        updates,
    }
}

fn is_correctly_ordered(rules: &[OrderingRule], update: &PageNumbers) -> bool {
    for rule in rules {
        let x = rule.x;
        let y = rule.y;

        let x_index = update.iter().position(|n| *n == x);
        let y_index = update.iter().position(|n| *n == y);

        let (Some(x_index), Some(y_index)) = (x_index, y_index) else {
            continue;
        };

        if x_index >= y_index {
            return false;
        }
    }

    true
}
