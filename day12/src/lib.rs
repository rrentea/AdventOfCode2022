use std::collections::{HashMap, VecDeque};

use nom::{
    character::complete::{alpha1, newline},
    multi::separated_list1,
    *,
};

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Node {
    x: i32,
    y: i32,
    height: u8,
}

type Graph = HashMap<Node, Vec<Node>>;

fn grid(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(
        newline,
        alpha1.map(|letters: &str| letters.chars().collect())
    )(input)
}

fn shortest_path(graph: &Graph, start: Node, goal: Node) -> Option<i32> {
    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut path = HashMap::new();
    path.insert(start, vec![]);


    while let Some(node) = queue.pop_front() {
        if node == goal {
            return Some(path[&goal].clone().len() as i32);
        }

        for neighbor in &graph[&node] {
            if !path.contains_key(neighbor) {
                queue.push_back(*neighbor);
                let mut new_path = path[&node].clone();
                new_path.push(*neighbor);
                path.insert(*neighbor, new_path);
            }
        }
    }
    None
}

pub fn part1(input: &str) -> String {
    let (_, grid) = grid(input).unwrap();

    let mut start = Node { x: 0, y: 0, height: 1 };
    let mut goal = Node { x: 0, y: 0, height: 26 };

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                start.x = i as i32;
                start.y = j as i32;
            } else if grid[i][j] == 'E' {
                goal.x = i as i32;
                goal.y = j as i32;
            }
        }
    }

    let grid = grid.iter()
        .map(|line| {
            line.iter()
                .map(|c| {
                    match c {
                        'S' => 1,
                        'E' => 26,
                        _ => *c as u8 - 'a' as u8 + 1
                    }
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    let mut graph = Graph::new();
    for i in 0i32..(grid.len() as i32) {
        for j in 0i32..(grid[0].len() as i32) {
            let curr_node = Node {
                x: i as i32,
                y : j as i32,
                height: grid[i as usize][j as usize]
            };
            let neighbors = vec![
                (i + 1, j),
                (i - 1, j),
                (i, j + 1),
                (i, j - 1),
            ];
            let edges = neighbors.iter()
                .filter_map(|(x, y)| {
                    grid.get(*x as usize)
                        .and_then(|vec| {
                            vec.get(*y as usize)
                        })
                        .and_then(|existing_cell| {
                            if curr_node.height + 1 >= *existing_cell {
                                Some(Node{
                                    x: *x,
                                    y: *y,
                                    height: *existing_cell})
                            } else {
                                None
                            }
                        })
                })
                .collect::<Vec<_>>();
            graph.insert(curr_node, edges);
        }
    }
    shortest_path(&graph, start, goal).unwrap().to_string()
}

pub fn part2(input: &str) -> String {
    let (_, grid) = grid(input).unwrap();

    let mut start_points = vec![];
    let mut goal = Node { x: 0, y: 0, height: 26 };

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' || grid[i][j] == 'a' {
                start_points.push(Node {
                    x: i as i32,
                    y: j as i32,
                    height: 1
                })
            } else if grid[i][j] == 'E' {
                goal.x = i as i32;
                goal.y = j as i32;
            }
        }
    }

    let grid = grid.iter()
        .map(|line| {
            line.iter()
                .map(|c| {
                    match c {
                        'S' => 1,
                        'E' => 26,
                        _ => *c as u8 - 'a' as u8 + 1
                    }
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    let mut graph = Graph::new();
    for i in 0i32..(grid.len() as i32) {
        for j in 0i32..(grid[0].len() as i32) {
            let curr_node = Node {
                x: i as i32,
                y : j as i32,
                height: grid[i as usize][j as usize]
            };
            let neighbors = vec![
                (i + 1, j),
                (i - 1, j),
                (i, j + 1),
                (i, j - 1),
            ];
            let edges = neighbors.iter()
                .filter_map(|(x, y)| {
                    grid.get(*x as usize)
                        .and_then(|vec| {
                            vec.get(*y as usize)
                        })
                        .and_then(|existing_cell| {
                            if curr_node.height + 1 >= *existing_cell {
                                Some(Node{
                                    x: *x,
                                    y: *y,
                                    height: *existing_cell})
                            } else {
                                None
                            }
                        })
                })
                .collect::<Vec<_>>();
            graph.insert(curr_node, edges);
        }
    }

    start_points.iter()
        .map(|point| {
            let dist = shortest_path(&graph, *point, goal);
            if dist.is_some() {
                dist.unwrap()
            } else {
                9999
            }
        })
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "31");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "29");
    }
}
