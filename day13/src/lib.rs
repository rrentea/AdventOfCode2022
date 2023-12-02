#![feature(iter_intersperse)]

use nom::{
    branch::alt,
    multi::separated_list0,
    multi::separated_list1,
    *, bytes::complete::tag, sequence::{delimited, separated_pair}, character::complete::newline,
};
use std::{cmp::Ordering, vec, fmt::Display};

#[derive(Debug, Eq)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

#[derive(Debug)]
struct Pair {
    left: Packet,
    right: Packet
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Packet::List(l1) => format!(
                    "[{}]",
                    l1.iter()
                      .map(|v| v.to_string())
                      .intersperse(",".to_string())
                      .collect::<String>()
                ),
                Packet::Integer(n) => n.to_string()

            }
        )
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Packet::Integer(n1), Packet::Integer(n2)) => n1 == n2,
            (Packet::List(l1), Packet::Integer(n2)) => l1 == &vec![Packet::Integer(*n2)],
            (Packet::Integer(n1), Packet::List(l2)) => &vec![Packet::Integer(*n1)] == l2,
            (Packet::List(l1), Packet::List(l2)) => l1 == l2
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(n1), Packet::Integer(n2)) => n1.cmp(n2),
            (Packet::List(l1), Packet::Integer(n2)) => l1.cmp(&vec![Packet::Integer(*n2)]),
            (Packet::Integer(n1), Packet::List(l2)) => vec![Packet::Integer(*n1)].cmp(&l2),
            (Packet::List(l1), Packet::List(l2)) => l1.cmp(l2)
        }
    }
}

fn list(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(
            tag("["),
            separated_list0(tag(","), list),
            tag("]"),
        )
        .map(|vec| Packet::List(vec)),
        nom::character::complete::u32
            .map(|num| Packet::Integer(num))
    ))(input)
}

fn pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(list, newline, list).map(
            |(p1, p2)| Pair { left: p1, right: p2 }
        )
    )(input)
}

pub fn part1(input: &str) -> String {
    let (_input, pairs) = pairs(input).unwrap();
    pairs.iter()
        .enumerate()
        .filter_map(|(i, Pair { left, right })| {
            if left < right {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum::<usize>()
        .to_string()

}

pub fn part2(input: &str) -> String {
    let (_input, pairs) = pairs(input).unwrap();
    let packet_2 = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    let packet_6 = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);

    let mut packets = pairs.iter()
        .flat_map(|Pair { left, right }| [left, right])
        .chain([&packet_2, &packet_6])
        .collect::<Vec<&Packet>>();
    packets.sort();
    println!(
        "{}",
        &packets
            .iter()
            .map(|v| v.to_string())
            .intersperse("\n".to_string())
            .collect::<String>()
    );
    let packet_2_idx = packets.iter()
        .enumerate()
        .map(|(i, v)| (i + 1, v))
        .find(|(_i, packet)| packet == &&&packet_2)
        .unwrap();
    let packet_6_idx = packets.iter()
        .enumerate()
        .map(|(i, v)| (i + 1, v))
        .find(|(_i, packet)| packet == &&&packet_6)
        .unwrap();
    (packet_2_idx.0 * packet_6_idx.0).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "140");
    }

}
