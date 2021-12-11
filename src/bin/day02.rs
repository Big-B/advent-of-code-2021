use advent_of_code_2021::*;

fn main() {
    let input = get_input().unwrap();
    let input: Vec<(&str, i64)> = input
        .lines()
        .map(|l| {
            let l: Vec<&str> = l.split_whitespace().collect();
            (l[0], l[1].parse::<i64>().unwrap())
        })
        .collect();

    println!("1. {}", calc_position(&input).multiply());
    println!("2. {}", calc_better_position(&input).multiply());
}

struct Position {
    horizontal: i64,
    depth: i64,
    aim: i64,
}

impl Position {
    fn new() -> Self {
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn multiply(&self) -> i64 {
        self.horizontal * self.depth
    }
}

fn calc_position(input: &[(&str, i64)]) -> Position {
    let mut pos = Position::new();

    for (direction, distance) in input {
        match *direction {
            "forward" => {
                pos.horizontal += distance;
            }
            "down" => {
                pos.depth += distance;
            }
            "up" => {
                pos.depth -= distance;
            }
            _ => {
                panic!("Unsupported direction")
            }
        }
    }

    pos
}

fn calc_better_position(input: &[(&str, i64)]) -> Position {
    let mut pos = Position::new();

    for (direction, distance) in input {
        match *direction {
            "forward" => {
                pos.horizontal += distance;
                pos.depth += distance * pos.aim;
            }
            "down" => {
                pos.aim += distance;
            }
            "up" => {
                pos.aim -= distance;
            }
            _ => {
                panic!("Unsupported direction");
            }
        }
    }

    pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation() {
        let vec = vec![
            ("forward", 5),
            ("down", 5),
            ("forward", 8),
            ("up", 3),
            ("down", 8),
            ("forward", 2),
        ];

        assert_eq!(calc_position(&vec).multiply(), 150);
    }

    #[test]
    fn test_better_navigation() {
        let vec = vec![
            ("forward", 5),
            ("down", 5),
            ("forward", 8),
            ("up", 3),
            ("down", 8),
            ("forward", 2),
        ];
        assert_eq!(calc_better_position(&vec).multiply(), 900);
    }
}
