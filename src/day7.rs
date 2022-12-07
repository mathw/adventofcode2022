use std::{error::Error, fmt::Display, iter, str::FromStr};

use regex::Regex;

use crate::day::{Day, DayResult, PartResult};

pub struct Day7 {
    input: &'static str,
}

impl Day7 {
    pub fn new() -> Day7 {
        Day7 {
            input: include_str!("inputs/day7.txt"),
        }
    }
}

impl Day for Day7 {
    fn run(&mut self) -> Result<DayResult, Box<dyn Error>> {
        let part1_result = run_part1(self.input)?;
        Ok(DayResult::new(
            PartResult::Success(part1_result.to_string()),
            PartResult::NotImplemented,
        ))
    }
}

fn run_part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let instructions = parse_input(input)?;
    let tree = build_directory_tree(&instructions)?;
    let small = tree.find_directories(|d| d.total_size() <= 100000);
    let sum = small.map(|d| d.total_size()).sum();
    Ok(sum)
}

#[derive(PartialEq, Eq, Debug)]
enum Command<'a> {
    CDIn(&'a str),
    CDOut,
    CDRoot,
    List,
}

#[derive(PartialEq, Eq, Debug)]
enum DirEntry<'a> {
    /// Directory with name
    Directory(&'a str),
    /// File with name and size
    File(&'a str, usize),
}

#[derive(PartialEq, Eq, Debug)]
enum InputLine<'a> {
    Command(Command<'a>),
    Result(DirEntry<'a>),
}

fn parse_input_line<'a>(line: &'a str) -> Result<InputLine<'a>, Box<dyn Error>> {
    lazy_static! {
        static ref FILE_REGEX: Regex = Regex::new(r"^(\d+) (.+)$").unwrap();
    }

    let line = line.trim();
    if line == "$ cd /" {
        Ok(InputLine::Command(Command::CDRoot))
    } else if line == "$ ls" {
        Ok(InputLine::Command(Command::List))
    } else if line == "$ cd .." {
        Ok(InputLine::Command(Command::CDOut))
    } else if line.starts_with("$ cd ") {
        Ok(InputLine::Command(Command::CDIn(&line[5..])))
    } else if line.starts_with("dir ") {
        Ok(InputLine::Result(DirEntry::Directory(&line[4..])))
    } else {
        let mut parts = line.split_whitespace();
        let size = parts
            .next()
            .ok_or_else(|| format!("Input line '{}' could not be parsed", line))?;
        let name = parts.next().ok_or_else(|| {
            format!(
                "Input line '{}' could not be parsed: expecting file entry, but no second part",
                line
            )
        })?;
        let size = usize::from_str(size)?;
        Ok(InputLine::Result(DirEntry::File(name, size)))
    }
}

fn parse_input<'a>(input: &'a str) -> Result<Vec<InputLine<'a>>, Box<dyn Error>> {
    input.lines().map(parse_input_line).collect()
}

#[derive(PartialEq, Eq, Debug)]
struct Directory<'a> {
    name: &'a str,
    files: Vec<File<'a>>,
    directories: Vec<Directory<'a>>,
}

impl<'a> Directory<'a> {
    fn new_empty(name: &'a str) -> Directory<'a> {
        Directory {
            name,
            files: Vec::new(),
            directories: Vec::new(),
        }
    }

    fn add_subdirectory(&mut self, name: &'a str) -> bool {
        if !self.knows_directory(name) {
            self.directories.push(Directory::new_empty(name));
            true
        } else {
            false
        }
    }

    fn knows_directory(&self, name: &'a str) -> bool {
        self.directories.iter().any(|d| d.name == name)
    }

    fn add_file(&mut self, name: &'a str, size: usize) {
        self.files.push(File::new(name, size))
    }

    fn find_directory_mut(&mut self, path: &[&'a str]) -> Option<&mut Directory<'a>> {
        let mut current = self;
        for p in path {
            current = current.get_directory_mut(p)?;
        }
        Some(current)
    }

    fn get_directory_mut(&mut self, name: &'a str) -> Option<&mut Directory<'a>> {
        self.directories.iter_mut().find(|d| d.name == name)
    }

    fn total_size(&self) -> usize {
        self.files.iter().map(|f| f.size).sum::<usize>()
            + self
                .directories
                .iter()
                .map(|d| d.total_size())
                .sum::<usize>()
    }

    fn find_directories(
        &'a self,
        f: impl Fn(&Self) -> bool + 'a,
    ) -> impl Iterator<Item = &Directory> {
        self.iterate_directories().filter(move |d| f(d))
    }

    fn iterate_directories(&'a self) -> impl Iterator<Item = &Directory> {
        self.directories.iter().flat_map(|d| {
            iter::once(d)
                .chain(d.iterate_directories())
                .collect::<Vec<_>>() // this gets around a recursive opaque type issue - appears to be a language limitation or me being silly
        })
    }
}

impl<'a> Display for Directory<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} (dir)", self.name)?;
        for d in self.directories.iter() {
            writeln!(f, "{}", d)?;
        }
        for file in self.files.iter() {
            writeln!(f, "{} ({} bytes)", file.name, file.size)?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug)]
struct File<'a> {
    name: &'a str,
    size: usize,
}

impl<'a> File<'a> {
    fn new(name: &'a str, size: usize) -> File<'a> {
        File { name, size }
    }
}

fn build_directory_tree<'a>(input: &[InputLine<'a>]) -> Result<Directory<'a>, Box<dyn Error>> {
    let mut root = Directory {
        name: "/",
        files: Vec::new(),
        directories: Vec::new(),
    };

    let mut current_path = Vec::new();

    for i in input {
        match i {
            InputLine::Result(DirEntry::File(name, size)) => {
                root.find_directory_mut(&current_path)
                    .ok_or_else(|| format!("Unable to find directory {:?}", current_path))?
                    .add_file(name, *size);
            }
            InputLine::Result(DirEntry::Directory(name)) => {
                let result = root
                    .find_directory_mut(&current_path)
                    .ok_or_else(|| format!("Unable to find directory {:?}", current_path))?
                    .add_subdirectory(name);
                if !result {
                    return Err(format!(
                        "Unable to add directory {} as there already was one",
                        name
                    )
                    .into());
                }
            }
            InputLine::Command(Command::CDIn(name)) => current_path.push(name),
            InputLine::Command(Command::CDOut) => {
                current_path.pop().ok_or_else(|| {
                    "Unable to cd .. because already at the root directory".to_string()
                })?;
            }
            InputLine::Command(Command::CDRoot) => current_path.clear(),
            InputLine::Command(Command::List) => {}
        }
    }

    Ok(root)
}

#[test]
fn test_parse_input() {
    let input = "$ cd /
$ ls
dir a
34 a.txt
$ cd a
$ cd ..";
    let parsed = parse_input(input).expect("Should parse");
    assert_eq!(
        parsed,
        vec![
            InputLine::Command(Command::CDRoot),
            InputLine::Command(Command::List),
            InputLine::Result(DirEntry::Directory("a")),
            InputLine::Result(DirEntry::File("a.txt", 34)),
            InputLine::Command(Command::CDIn("a")),
            InputLine::Command(Command::CDOut)
        ]
    );
}

#[test]
fn test_build_directory_simple() {
    let input = parse_input(
        "$ cd /
$ ls
dir a
$ cd a
$ ls
228 tribble.exe",
    )
    .expect("Should parse");

    let dir = build_directory_tree(&input).expect("shouldn't crash");

    assert_eq!(dir.name, "/");
    assert_eq!(dir.files, vec![]);
    assert_eq!(dir.directories.len(), 1);
    let dir_a = &dir.directories[0];
    assert_eq!(dir_a.directories, vec![]);
    assert_eq!(dir_a.files.len(), 1);
    let file_tribble = &dir_a.files[0];
    assert_eq!(file_tribble.name, "tribble.exe");
    assert_eq!(file_tribble.size, 228);
}

#[test]
fn test_total_size_simple() {
    let input = parse_input(
        "$ cd /
$ ls
dir a
$ cd a
$ ls
228 tribble.exe",
    )
    .expect("Should parse");

    let dir = build_directory_tree(&input).expect("shouldn't crash");

    assert_eq!(dir.total_size(), 228);
}

#[test]
fn test_total_size_nested() {
    let input = parse_input(
        "$ cd /
$ ls
dir a
$ cd a
$ ls
228 tribble.exe
dir b
$ cd b
$ ls
2 a.txt",
    )
    .expect("Should parse");

    let dir = build_directory_tree(&input).expect("shouldn't crash");

    assert_eq!(dir.total_size(), 230);
    assert_eq!(dir.directories[0].directories[0].total_size(), 2);
}

#[test]
fn test_part1_sample() {
    let input = "$ cd /
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
    let result = run_part1(input).expect("This should work");
    assert_eq!(result, 95437);
}
