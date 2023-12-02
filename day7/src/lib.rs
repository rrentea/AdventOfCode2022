use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::{
        complete::{alpha1, anychar, newline, one_of},
        is_alphabetic,
    },
    multi::{many1, separated_list1},
    sequence::separated_pair,
    *,
};

#[derive(Debug)]
struct File<'a> {
    size: u32,
    name: &'a str,
}

#[derive(Debug)]
enum Files<'a> {
    File {size: u32, name: &'a str},
    Dir(&'a str)
}

#[derive(Debug)]
enum Operation<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Files<'a>>)
}

#[derive(Debug)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

fn file(input: &str) -> IResult<&str, Files> {
    let (input, (size, name)) = separated_pair(
        nom::character::complete::u32,
        tag(" "),
        is_a("qwertyuiopasdfghjklzxcvbnm.")
    )(input)?;

    Ok((input, Files::File {size, name}))
}

fn directory(input: &str) -> IResult<&str, Files> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, Files::Dir(name)))
}

fn ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((
        file, directory
    )))(input)?;

    Ok((input, Operation::Ls(files)))
}

fn cd(input: &str) -> IResult<&str, Operation> {
    let (input, _cd) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), alpha1, tag("/")))(input)?;

    let op = match dir {
        ".." => Operation::Cd(Cd::Up),
        "/"  => Operation::Cd(Cd::Root),
        name => Operation::Cd(Cd::Down(name))
    };

    Ok((input, op))
}

fn commands(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, cmd) =
        separated_list1(newline, alt((ls, cd)))(input)?;

    Ok((input, cmd))
}

pub fn process_part1(input: &str) -> String {
    let (_input, cmds) = commands(input).unwrap();
    let mut directories: BTreeMap<String, Vec<File>> = BTreeMap::new();
    let mut context: Vec<String> = vec![]; 

    for cmd in cmds.iter() {
        match cmd {
            Operation::Cd(Cd::Root) => {
                context.push("".to_string());
            },
            Operation::Cd(Cd::Up) => {
                context.pop();
            },
            Operation::Cd(Cd::Down(name)) => {
                context.push(name.to_string());
            },
            Operation::Ls(files) => {
                directories
                    .entry(
                        context
                            .iter()
                            .cloned()
                            .collect::<Vec<String>>()
                            .join("/")
                    )
                    .or_insert(vec![]);
                for file in files.iter() {
                    match file {
                        Files::File { size, name } => {
                            directories.entry(
                                context
                                    .iter()
                                    .cloned()
                                    .collect::<Vec<String>>()
                                    .join("/")
                                )
                                .and_modify(|vec| {
                                    vec.push(File {
                                        size: *size,
                                        name
                                    })
                                });
                        }
                        Files::Dir(_) => (),
                    }
                }
            },
        };
    }

    let mut sizes: BTreeMap<String, u32> = BTreeMap::new();
    for (path, files) in directories.iter() {
        let dirs = path.split("/").collect::<Vec<&str>>();
        let size = files
            .iter()
            .map(|File {size, ..} | size)
            .sum::<u32>();
        for i in 0..dirs.len() {
            sizes
                .entry(
                    (&dirs[0..=i])
                        .iter()
                        .cloned()
                        .collect::<Vec<&str>>()
                        .join("/")
                )
                .and_modify(|v| *v += size)
                .or_insert(size);
        }
    }

    sizes
        .iter()
        .filter(|(_, &size)| size < 100000)
        .map(|(_, size)| size)
        .sum::<u32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_input, cmds) = commands(input).unwrap();
    let mut directories: BTreeMap<String, Vec<File>> = BTreeMap::new();
    let mut context: Vec<String> = vec![]; 

    for cmd in cmds.iter() {
        match cmd {
            Operation::Cd(Cd::Root) => {
                context.push("".to_string());
            },
            Operation::Cd(Cd::Up) => {
                context.pop();
            },
            Operation::Cd(Cd::Down(name)) => {
                context.push(name.to_string());
            },
            Operation::Ls(files) => {
                directories
                    .entry(
                        context
                            .iter()
                            .cloned()
                            .collect::<Vec<String>>()
                            .join("/")
                    )
                    .or_insert(vec![]);
                for file in files.iter() {
                    match file {
                        Files::File { size, name } => {
                            directories.entry(
                                context
                                    .iter()
                                    .cloned()
                                    .collect::<Vec<String>>()
                                    .join("/")
                                )
                                .and_modify(|vec| {
                                    vec.push(File {
                                        size: *size,
                                        name
                                    })
                                });
                        }
                        Files::Dir(_) => (),
                    }
                }
            },
        };
    }

    let mut sizes: BTreeMap<String, u32> = BTreeMap::new();
    for (path, files) in directories.iter() {
        let dirs = path.split("/").collect::<Vec<&str>>();
        let size = files
            .iter()
            .map(|File {size, ..} | size)
            .sum::<u32>();
        for i in 0..dirs.len() {
            sizes
                .entry(
                    (&dirs[0..=i])
                        .iter()
                        .cloned()
                        .collect::<Vec<&str>>()
                        .join("/")
                )
                .and_modify(|v| *v += size)
                .or_insert(size);
        }
    }

    let used_space: u32 = *sizes.get("").unwrap();
    let need_to_free: u32 = 30_000_000 - (70_000_000 - used_space);
    dbg!(used_space, need_to_free);

    let mut valid_dirs = sizes
        .iter()
        .filter(|(_, &size)| size > need_to_free)
        .map(|(_, size)| size)
        .collect::<Vec<&u32>>();
    
    valid_dirs.sort();
    valid_dirs.iter().next().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "95437");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "24933642");
    }
}
