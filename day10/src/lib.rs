use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Op {
    Noop,
    Addx
}

#[derive(Clone, Copy, Debug)]
struct Command {
    operation: Op,
    arg: i32
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        match parts[0] {
            "noop" => {
                Ok(Command { operation: Op::Noop, arg: 0})
            },
            "addx" => {
                let arg = parts[1].parse::<i32>().expect(&"Invalid arg".to_string());
                Ok(Command { operation: Op::Addx, arg})
            },
            _ => return Err("Unknown direction".to_string()),
        }
    }
}

pub fn part1(input: &str) -> String {
    let commands: Vec<Command> = input
        .lines()
        .map(|s| s.parse::<Command>().unwrap())
        .collect();
    let mut signal_strengths: Vec<i32> = vec![];
    let mut x_register = 1;

    for command in commands {
        match command.operation {
            Op::Addx => {
                signal_strengths.push(x_register);
                signal_strengths.push(x_register);
                x_register += command.arg;
            },
            Op::Noop => {
                signal_strengths.push(x_register);
            }
        }
    }

    let mut i = 20;
    let mut sum = 0;
    while i < signal_strengths.len() {
        dbg!(signal_strengths[i]);
        sum += i as i32 * signal_strengths[i - 1];
        i += 40;
    }
    sum.to_string()
}

pub fn part2(input: &str) -> String {
    let commands: Vec<Command> = input
        .lines()
        .map(|s| s.parse::<Command>().unwrap())
        .collect();
    let mut signal_strengths: Vec<i32> = vec![];
    let mut x_register = 1;

    for command in commands {
        match command.operation {
            Op::Addx => {
                signal_strengths.push(x_register);
                signal_strengths.push(x_register);
                x_register += command.arg;
            },
            Op::Noop => {
                signal_strengths.push(x_register);
            }
        }
    }

    let mut screen: String = String::new();
    for c in 0..6 {
        for i in 0..40 {
            let sprite_position = signal_strengths[40 * c + i];

            if ((sprite_position-1)..=(sprite_position+1)).contains(&(i as i32)) {
                screen.push('#');
            } else {
                screen.push('.');
            }
        }
        screen.push('\n');
    }
    screen
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    const PART2_OUTPUT: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "13140");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, PART2_OUTPUT);
    }
}
