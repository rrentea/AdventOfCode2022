use std::{str::FromStr, collections::HashSet, vec};

#[derive(Clone, Copy, Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down
}

#[derive(Clone, Copy, Debug)]
struct Command {
    direction: Direction,
    count: u32,
}


impl FromStr for Command {

    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        let direction = match parts[0] {
            "R" => Direction::Right,
            "U" => Direction::Up,
            "L" => Direction::Left,
            "D" => Direction::Down,
            _ => return Err("Unknown direction".to_string()),
        };

        let count = parts[1].parse::<u32>().expect(&"Invalid counts".to_string());

        Ok(Command { direction, count })
    }
}

fn distance(a: (i32, i32), b: (i32, i32)) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
}

fn get_direction(knot1: (i32, i32), knot2: (i32, i32)) -> (i32, i32) {
    if knot1.0 > knot2.0 && knot1.1 > knot2.1 {
        (1, 1) // NE
    } else if knot1.0 > knot2.0 && knot1.1 < knot2.1 {
        (1, -1) // NW
    } else if knot1.0 < knot2.0 && knot1.1 > knot2.1 {
        (-1, 1) // SE
    } else if knot1.0 < knot2.0 && knot1.1 < knot2.1 {
        (-1, -1) // SW
    } else {
        let mut x_off = knot1.0 - knot2.0;
        let mut y_off = knot1.1 - knot2.1;
        if x_off > 0 {
            x_off = 1;
        } else if x_off < 0 {
            x_off = -1;
        }
        if y_off > 0 {
            y_off = 1;
        } else if y_off < 0 {
            y_off = -1;
        }
        (x_off, y_off)
    }
}

fn move_knot(knot1: (i32, i32), knot2: (i32, i32)) -> (i32, i32) {
    let dist = distance(knot1, knot2);
    let is_on_same_line = knot1.0 == knot2.0 || knot1.1 == knot2.1;

    if (is_on_same_line && dist > 1) || 
        (!is_on_same_line && dist > 2)
    {
        let direction = get_direction(knot1, knot2);
        return (knot2.0 + direction.0, knot2.1 + direction.1);
    }
    knot2 
}

pub fn part1(input: &str) -> String {
    let commands: Vec<Command> = input
        .lines()
        .map(|s| s.parse::<Command>().unwrap())
        .collect();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);

    visited.insert(tail);
    for Command { direction, count } in commands.iter() {
        for _ in 0..*count {
            match direction {
                Direction::Up => head.0 += 1,
                Direction::Down => head.0 -= 1,
                Direction::Right => head.1 += 1,
                Direction::Left => head.1 -= 1,
            }
            tail = move_knot(head, tail);
            visited.insert(tail);
        }
    }

    visited.len().to_string()
}


pub fn part2(input: &str) -> String {

    let commands: Vec<Command> = input
        .lines()
        .map(|s| s.parse::<Command>().unwrap())
        .collect();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut rope: Vec<(i32, i32)> = vec![];
    for _ in 0..10 {
        rope.push((0, 0));
    }

    visited.insert(rope[9]);
    for Command { direction, count } in commands.iter() {
        for _ in 0..*count {
            // Move the head
            match direction {
                Direction::Up => rope[0].0 += 1,
                Direction::Down => rope[0].0 -= 1,
                Direction::Right => rope[0].1 += 1,
                Direction::Left => rope[0].1 -= 1,
            }
            
            for i in 1..10 {
                rope[i] = move_knot(rope[i-1], rope[i]);
            }
            visited.insert(rope[9]);
        }
    }

    visited.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const INPUT_PART1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT_PART2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part1_works() {
        let result = part1(INPUT_PART1);
        assert_eq!(result, "13");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT_PART2);
        assert_eq!(result, "36");
    }
}
