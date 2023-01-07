use std::collections::{HashMap,VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub fn part1() -> i32 {
    make_dir_sizes()
        .values()
        .filter(|&&size| size <= 100000)
        .sum()
}

pub fn part2() -> i32 {
    let dir_sizes = make_dir_sizes();
    let total_fs_size = 70000000;
    let root_size: i32 = dir_sizes["/"];
    let free_space = total_fs_size - root_size;
    let update_space = 30000000;
    assert!(free_space < update_space);
    let space_needed = update_space - free_space;
    dir_sizes.values()
        .filter(|&&size| size >= space_needed)
        .min()
        .copied()
        .unwrap()
}

fn make_dir_sizes() -> HashMap<String, i32> {
    let mut reader = Reader::new();
    let mut commands: Vec<Command> = vec![];

    loop {
        let line = reader.consume_line();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();
        assert!(line.starts_with("$ "));
        let words = line.split(' ')
            .into_iter().collect::<Vec<&str>>();
        let command_name: &str = words[1];

        let command: Command = match command_name {
            "cd" => Command::Cd { dest: words[2].to_string() },
            "ls" => parse_ls(&mut reader),
            _ => unimplemented!("{}", command_name)
        };

        commands.push(command);
    }

    let mut current_path = FictionalPath::new();
    let mut dir_sizes = HashMap::<String, i32>::new();

    for command in commands {
        match command {
            Command::Cd { ref dest } => {
                current_path.push(dest);
            }
            Command::Ls { ref output } => {
                for o in output {
                    if let LsOutputFileInfo::FileSize(size) = o.file_info {
                        dir_sizes.insert(
                            "/".to_string(),
                            dir_sizes.get("/").unwrap_or(&0) + size
                        );

                        for i in 0..current_path.comps.len() {
                            let slice = &current_path.comps[0..=i];
                            let key = format!("/{}", slice.join("/"));
                            assert!(!key.is_empty());
                            let value = dir_sizes.get(&key).unwrap_or(&0)
                                + size;
                            dir_sizes.insert(key, value);
                        }
                    }
                }
            }
        }
    }

    dir_sizes
}

// An absolute path that doesn't exist on the file system
struct FictionalPath {
    comps: Vec<String>
}

impl FictionalPath {
    fn new() -> Self {
        Self { comps: vec![] }
    }

    fn push(&mut self, comp: &str) {
        match comp {
            "/" => {
                assert!(self.comps.len() == 0);
            }
            ".." => {
                self.comps.pop().unwrap();
            }
            _ => {
                self.comps.push(comp.to_string());
            }
        }
    }
}

impl std::fmt::Display for FictionalPath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.comps.len() == 0 {
            write!(f, "/")
        } else {
            for comp in self.comps.iter() {
                write!(f, "/{}", comp)?;
            }

            Ok(())
        }
    }
}

#[derive(Debug)]
enum Command {
    Cd { dest: String },
    Ls { output: Vec<LsOutputEntry> }
}

#[derive(Debug)]
struct LsOutputEntry {
    file_info: LsOutputFileInfo
}

#[derive(Debug)]
enum LsOutputFileInfo {
    FileSize(i32),
    FileIsDir
}

fn parse_ls(reader: &mut Reader) -> Command {
    let mut output = Vec::<LsOutputEntry>::new();

    loop {
        let output_line = reader.peek_line();

        if let Some(ref output_line) = output_line {
            if output_line.starts_with("$ ") {
                break;
            } else {
                reader.consume_line();

                let output_line_segments: Vec<&str> = output_line.split(' ')
                    .into_iter().collect();
                let file_info: LsOutputFileInfo =
                    output_line_segments[0].parse::<i32>()
                    .map(|filesize| LsOutputFileInfo::FileSize(filesize))
                    .unwrap_or(LsOutputFileInfo::FileIsDir);
                output.push(
                    LsOutputEntry {
                        file_info
                    }
                );
            }
        } else {
            break;
        }
    }

    assert!(!output.is_empty());
    Command::Ls { output }
}

struct Reader {
    lines: Lines<BufReader<File>>,
    peeked_lines: VecDeque<String>
}

impl Reader {
    fn new() -> Self {
        let file = std::fs::File::open("input7.txt").unwrap();
        let lines = std::io::BufReader::new(file).lines();
        Reader {
            lines,
            peeked_lines: VecDeque::new()
        }
    }

    fn peek_line(&mut self) -> Option<String> {
        if self.peeked_lines.len() > 0 {
            self.peeked_lines.pop_front()
        } else {
            let line = self.consume_line();

            if let Some(ref line) = line {
                self.peeked_lines.push_back(line.clone());
            }

            line
        }
    }

    fn consume_line(&mut self) -> Option<String> {
        if self.peeked_lines.len() > 0 {
            self.peeked_lines.pop_front()
        } else {
            self.lines.next().map(|result| result.unwrap())
        }
    }
}
