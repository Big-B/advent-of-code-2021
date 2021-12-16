use advent_of_code_2021::*;
fn main() {
    let input = get_input().unwrap();
    println!("80 Days Spawn: {}", run_spawn_sim(&input, 80));
    println!("256 Days Spawn: {}", run_spawn_sim(&input, 256));
}

fn run_spawn_sim(input: &str, days: usize) -> usize {
    let input: Vec<usize> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut fish = [0; 9];
    for f in input {
        fish[f] += 1;
    }

    for _ in 0..days {
        let size = fish.len();
        let copy = fish;
        fish[..size-1].clone_from_slice(&copy[1..size]);
        fish[6] += copy[0];
        fish[8] = copy[0];
    }

    fish.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_80_days() {
        let input = "3,4,3,1,2";
        assert_eq!(run_spawn_sim(input, 80), 5934);
    }

    #[test]
    fn test_256_days() {
        let input = "3,4,3,1,2";
        assert_eq!(run_spawn_sim(input, 256), 26984457539);
    }
}
