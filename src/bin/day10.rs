use advent_of_code_2021::*;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = get_input().unwrap();
    println!("Corrupted Score: {}", corrupted_score(&input));
    println!("Remainder Score: {}", remainder_score(&input));
}

fn corrupted_score(input: &str) -> u64 {
    input.lines().map(corrupted_line).sum()
}

fn corrupted_line(input: &str) -> u64 {
    let mut stack = Vec::new();
    let mut score = 0;
    let map = HashMap::from([(')', '('), (']', '['), ('>', '<'), ('}', '{')]);
    let score_map = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let opener: HashSet<char> = map.values().cloned().collect();
    for c in input.chars() {
        if opener.contains(&c) {
            stack.push(c);
        } else if map.get(&c).unwrap() == stack.last().unwrap() {
            stack.pop();
        } else {
            score = *score_map.get(&c).unwrap();
            break;
        }
    }
    score
}

fn remainder_score(input: &str) -> u64 {
    let mut input: Vec<u64> = input
        .lines()
        .filter(|x| corrupted_score(x) == 0)
        .map(remainder_line)
        .collect();
    input.sort_unstable();
    input[input.len() / 2]
}

fn remainder_line(input: &str) -> u64 {
    let mut stack = Vec::new();
    let map = HashMap::from([(')', '('), (']', '['), ('>', '<'), ('}', '{')]);
    let score_map = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let opener: HashSet<char> = map.values().cloned().collect();
    for c in input.chars() {
        if opener.contains(&c) {
            stack.push(c);
        } else {
            stack.pop();
        }
    }

    let mut score = 0;
    while !stack.is_empty() {
        score *= 5;
        score += score_map.get(&stack.pop().unwrap()).unwrap();
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corrupted_score() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(corrupted_score(input), 26397);
    }

    #[test]
    fn test_basin_size() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(remainder_score(input), 288957);
    }
}
