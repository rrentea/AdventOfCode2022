use std::{collections::VecDeque, fmt::Display};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace1, newline},
    multi::separated_list1,
    sequence::{delimited, preceded},
    *
};

#[derive(Debug)]
enum Value {
    Old,
    Num(u64),
}

#[derive(Debug)]
enum Operation {
    Mul((Value, Value)),
    Add((Value, Value)),
}

#[derive(Debug)]
struct Test {
    divisible: u64,
    true_recipient: u64,
    false_recipient: u64,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    touch_count: u64,
}

impl Monkey {
    fn inspect(
        &mut self,
        lower_worry_level: bool,
        magic_trick: u64
    ) -> u64{
        self.touch_count += 1;
        let item = self.items.pop_front().unwrap();
        let worry_level = match &self.operation {
            Operation::Mul((_, b)) => {
                let num2 = match b {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };
                (item * num2) % magic_trick
            },
            Operation::Add((_, b)) => {
                let num2 = match b {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };
                (item + num2) % magic_trick
            }
        };
        if lower_worry_level {
            worry_level / 3
        } else {
            worry_level
        }
    }

    fn test(&self, new_item: u64) -> u64 {
        if new_item % self.test.divisible == 0 {
            self.test.true_recipient
        } else {
            self.test.false_recipient
        }
    }
}

fn value(input: &str) -> IResult<&str, Value> {
    alt((
        tag("old").map(|_| Value::Old),
        nom::character::complete::u64
            .map(|num| Value::Num(num))
    ))(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("Operation: new = ")(input)?;
    let (input, value_1) = value(input)?;
    let (input, operator) = delimited(
        multispace1,
        alt((tag("*"), tag("+"))),
        multispace1
    )(input)?;
    let (input, value_2) = value(input)?;

    let result = match operator {
        "*" => Operation::Mul((value_1, value_2)),
        "+" => Operation::Add((value_1, value_2)),
        _ => panic!("unknown operator"),
    };
    Ok((input, result))
}

fn test(input: &str) -> IResult<&str, Test> {
    let (input, divisible) = preceded(
        tag("Test: divisible by "),
        nom::character::complete::u64,
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, true_recipient) = preceded(
        tag("If true: throw to monkey "),
        nom::character::complete::u64,
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, false_recipient) = preceded(
        tag("If false: throw to monkey "),
        nom::character::complete::u64,
    )(input)?;

    Ok((
        input,
        Test {
            divisible,
            true_recipient,
            false_recipient,
        },
    ))
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _id) = delimited(
        tag("Monkey "),
        nom::character::complete::u64,
        tag(":"),
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, items) = preceded(
        tag("Starting items: "),
        separated_list1(
            tag(", "),
            nom::character::complete::u64,
        )
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, op) = operation(input)?;
    let (input, _) = multispace1(input)?;
    let (input, test) = test(input)?;

    Ok((
        input,
        Monkey {
            items: VecDeque::from(items),
            operation: op,
            test,
            touch_count: 0,
        }
    ))
}

pub fn part1(input: &str) -> String {
    let (_, mut monkeys) = separated_list1(tag("\n\n"), monkey)(input).unwrap();
    let magic_trick = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible)
        .product::<u64>();

    for _round in 0..20 {
        for monkey_index in 0..monkeys.len() {
            for _i in 0..monkeys[monkey_index].items.len() {
                let monkey = monkeys.get_mut(monkey_index).unwrap();
                let new_item = monkey.inspect(true, magic_trick);
                let recepient = monkey.test(new_item);
                monkeys
                    .get_mut(recepient as usize)
                    .unwrap()
                    .items
                    .push_back(new_item);
            }
        }
    }
    monkeys.sort_by_key(|monkey| monkey.touch_count);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.touch_count)
        .product::<u64>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let (_, mut monkeys) = separated_list1(tag("\n\n"), monkey)(input).unwrap();
    let magic_trick = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible)
        .product::<u64>();

    for _round in 0..10_000 {
        for monkey_index in 0..monkeys.len() {
            for _i in 0..monkeys[monkey_index].items.len() {
                let monkey = monkeys.get_mut(monkey_index).unwrap();
                let new_item = monkey.inspect(false, magic_trick);
                let recepient = monkey.test(new_item);
                monkeys
                    .get_mut(recepient as usize)
                    .unwrap()
                    .items
                    .push_back(new_item);
            }
        }
    }
    monkeys.sort_by_key(|monkey| monkey.touch_count);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.touch_count)
        .product::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "10605");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "2713310158");
    }
}
