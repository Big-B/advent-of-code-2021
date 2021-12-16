use advent_of_code_2021::*;
use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
    let input = get_input().unwrap();
    let lines = parse_into_lines(&input);

    println!("Num Straight Overlap: {}", number_of_straight_overlaps(&lines));
    println!("Num Overlap: {}", number_of_overlaps(&lines));
}

struct LineIter<'a> {
    line: &'a Line,
    step: usize,
    done: bool,
}

fn sign(start: i64, stop: i64) -> i64 {
    match stop.cmp(&start) {
        Ordering::Greater => 1,
        Ordering::Less => -1,
        Ordering::Equal => 0,
    }
}

impl Iterator for LineIter<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let x =
                self.step as i64 * sign(self.line.start.x, self.line.stop.x) + self.line.start.x;
            let y =
                self.step as i64 * sign(self.line.start.y, self.line.stop.y) + self.line.start.y;
            let point = Point::new(x, y);
            if point == self.line.stop {
                self.done = true;
            } else {
                self.step += 1;
            }
            Some(point)
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

struct Line {
    pub start: Point,
    pub stop: Point,
}

impl Line {
    pub fn is_straight(&self) -> bool {
        self.start.x == self.stop.x || self.start.y == self.stop.y
    }

    pub fn iter(&self) -> LineIter {
        LineIter {
            line: self,
            step: 0,
            done: false,
        }
    }
}

fn parse_into_lines(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|s| {
            let v: Vec<i64> = s
                .trim()
                .split(&['-', '>', ',', ' '][..])
                .filter(|s| !s.is_empty())
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            Line {
                start: Point::new(v[0], v[1]),
                stop: Point::new(v[2], v[3]),
            }
        })
        .collect()
}

fn number_of_straight_overlaps(lines: &[Line]) -> usize {
    let mut points: HashMap<Point, usize> = HashMap::new();
    for line in lines.iter().filter(|l| l.is_straight()) {
        for point in line.iter() {
            let entry = points.entry(point).or_insert(0);
            *entry += 1;
        }
    }

    points.iter().filter(|(_, &v)| v > 1).count()
}

fn number_of_overlaps(lines: &[Line]) -> usize {
    let mut points: HashMap<Point, usize> = HashMap::new();
    for line in lines.iter() {
        for point in line.iter() {
            let entry = points.entry(point).or_insert(0);
            *entry += 1;
        }
    }

    points.iter().filter(|(_, &v)| v > 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_straight_overlaps() {
        let input = "0,9 -> 5,9
                     8,0 -> 0,8
                     9,4 -> 3,4
                     2,2 -> 2,1
                     7,0 -> 7,4
                     6,4 -> 2,0
                     0,9 -> 2,9
                     3,4 -> 1,4
                     0,0 -> 8,8
                     5,5 -> 8,2";
        assert_eq!(number_of_straight_overlaps(&parse_into_lines(input)), 5);
    }

    #[test]
    fn test_num_overlaps() {
        let input = "0,9 -> 5,9
                     8,0 -> 0,8
                     9,4 -> 3,4
                     2,2 -> 2,1
                     7,0 -> 7,4
                     6,4 -> 2,0
                     0,9 -> 2,9
                     3,4 -> 1,4
                     0,0 -> 8,8
                     5,5 -> 8,2";
        assert_eq!(number_of_overlaps(&parse_into_lines(input)), 12);
    }
}
