use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

fn parse(input: &str) -> HashMap<PathBuf, usize> {
    let mut cwd = PathBuf::default();
    let mut sizes: HashMap<PathBuf, usize> = HashMap::default();
    let mut lines = input.lines().peekable();
    while let Some(line) = lines.next() {
        if line.starts_with('$') {
            match &line[2..4] {
                "cd" => match &line[5..] {
                    "/" => {
                        cwd = PathBuf::from("/");
                    }
                    ".." => {
                        cwd.pop();
                    }
                    path => {
                        cwd.push(path);
                    }
                },
                "ls" => {
                    while let Some(line) = lines.peek() {
                        if line.starts_with('$') {
                            break;
                        } else {
                            let line = lines.next().unwrap();
                            if !line.starts_with("dir") {
                                if let Some((size, _)) = line.split_once(' ') {
                                    let file_size = size.parse().unwrap_or_default();
                                    let mut ancestors = cwd.ancestors();
                                    while let Some(ancestor) = ancestors.next() {
                                        sizes
                                            .entry(ancestor.to_path_buf())
                                            .and_modify(|size| {
                                                *size += file_size;
                                            })
                                            .or_insert_with(|| file_size);
                                    }
                                }
                            }
                        }
                    }
                }
                _ => panic!("unknown command"),
            };
        }
    }
    sizes
}

fn one(input: &str) -> usize {
    parse(input).values().filter(|&&x| x <= 100_000).sum()
}

fn two(input: &str) -> usize {
    let sizes = parse(input);
    let used = sizes.get(Path::new("/")).unwrap();
    let unused = 70_000_000 - used;
    let needed = 30_000_000 - unused;
    sizes
        .values()
        .filter(|&&size| size >= needed)
        .min()
        .copied()
        .unwrap_or_default()
}

fn main() -> Result<(), std::io::Error> {
    let input = fs::read_to_string("input")?;
    println!("1: {}", one(&input));
    println!("2: {}", two(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"$ cd /
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
7214296 k
"#;

    #[test]
    fn part_one() {
        assert_eq!(one(INPUT), 95437);
    }

    #[test]
    fn part_two() {
        assert_eq!(two(INPUT), 24933642);
    }
}
