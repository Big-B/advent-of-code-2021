use advent_of_code_2021::*;
fn main() {
    let input = get_input().unwrap();
    println!("Fuel Cost: {}", most_fuel_efficient(&input));
    println!("Complex Fuel Cost: {}", most_fuel_efficient_complex(&input));
}

fn most_fuel_efficient(input: &str) -> i64 {
    let vec: Vec<i64> = input
        .trim()
        .split(',')
        .map(|v| v.parse::<i64>().unwrap())
        .collect();
    let mut min_fuel = i64::MAX;
    for i in 0.. {
        let fuel = vec.iter().map(|&v| (v - i).abs()).sum();
        if fuel < min_fuel {
            min_fuel = fuel;
        } else {
            break;
        }
    }

    min_fuel
}

fn most_fuel_efficient_complex(input: &str) -> i64 {
    let vec: Vec<i64> = input
        .trim()
        .split(',')
        .map(|v| v.parse::<i64>().unwrap())
        .collect();
    let mut min_fuel = i64::MAX;
    for i in 0.. {
        let fuel = vec
            .iter()
            .map(|&v| (v - i).abs())
            .map(|n| (n * (n + 1)) / 2)
            .sum();
        if fuel < min_fuel {
            min_fuel = fuel;
        } else {
            break;
        }
    }

    min_fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_fuel_efficient() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(most_fuel_efficient(input), 37);
    }

    #[test]
    fn test_256_days() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(most_fuel_efficient_complex(input), 168);
    }
}
