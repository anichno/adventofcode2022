use std::{cell::RefCell, rc::Rc, rc::Weak};

#[derive(Debug)]
enum Record {
    File(usize),
    Directory(Rc<RefCell<Directory>>),
}

#[derive(Debug)]
struct Directory {
    parent: Option<Weak<RefCell<Directory>>>,
    contents: Vec<Record>,
    tot_size: usize,
}

fn solve1(input: &[&str]) -> usize {
    let root = Rc::new(RefCell::new(Directory {
        parent: None,
        contents: Vec::new(),
        tot_size: 0,
    }));

    let mut total = 0;
    #[allow(clippy::redundant_clone)]
    let mut cur_dir = root.clone();
    for line in input {
        let mut parts = line.split_whitespace();
        let part = parts.next().unwrap();
        if part == "$" {
            // command
            if parts.next().unwrap() == "cd" {
                if parts.next().unwrap() == ".." {
                    let mut dir_tot = 0;
                    for record in cur_dir.borrow().contents.iter() {
                        match record {
                            Record::File(size) => dir_tot += size,
                            Record::Directory(dir) => dir_tot += dir.borrow().tot_size,
                        }
                    }
                    cur_dir.borrow_mut().tot_size = dir_tot;
                    if dir_tot <= 100000 {
                        total += dir_tot;
                    }
                    if cur_dir.borrow().parent.is_some() {
                        let parent = cur_dir.borrow().parent.as_ref().unwrap().upgrade().unwrap();
                        cur_dir = parent;
                    }
                } else {
                    let new_dir = Rc::new(RefCell::new(Directory {
                        parent: Some(Rc::downgrade(&cur_dir)),
                        contents: Vec::new(),
                        tot_size: 0,
                    }));
                    cur_dir
                        .borrow_mut()
                        .contents
                        .push(Record::Directory(new_dir.clone()));
                    cur_dir = new_dir;
                }
            }
        } else if part != "dir" {
            let size = part.parse().unwrap();
            cur_dir.borrow_mut().contents.push(Record::File(size));
        }
    }

    total
}

fn solve2(input: &[&str]) -> usize {
    let total_disk_space = 70000000;
    let needed_unused = 30000000;
    let root = Rc::new(RefCell::new(Directory {
        parent: None,
        contents: Vec::new(),
        tot_size: 0,
    }));

    let mut cur_dir = root.clone();
    let mut dir_sizes = Vec::new();
    for line in input {
        let mut parts = line.split_whitespace();
        let part = parts.next().unwrap();
        if part == "$" {
            // command
            if parts.next().unwrap() == "cd" {
                if parts.next().unwrap() == ".." {
                    let mut dir_tot = 0;
                    for record in cur_dir.borrow().contents.iter() {
                        match record {
                            Record::File(size) => dir_tot += size,
                            Record::Directory(dir) => dir_tot += dir.borrow().tot_size,
                        }
                    }
                    cur_dir.borrow_mut().tot_size = dir_tot;
                    dir_sizes.push(dir_tot);

                    if cur_dir.borrow().parent.is_some() {
                        let parent = cur_dir.borrow().parent.as_ref().unwrap().upgrade().unwrap();
                        cur_dir = parent;
                    }
                } else {
                    let new_dir = Rc::new(RefCell::new(Directory {
                        parent: Some(Rc::downgrade(&cur_dir)),
                        contents: Vec::new(),
                        tot_size: 0,
                    }));
                    cur_dir
                        .borrow_mut()
                        .contents
                        .push(Record::Directory(new_dir.clone()));
                    cur_dir = new_dir;
                }
            }
        } else if part != "dir" {
            let size = part.parse().unwrap();
            cur_dir.borrow_mut().contents.push(Record::File(size));
        }
    }

    loop {
        let mut dir_tot = 0;
        for record in cur_dir.borrow().contents.iter() {
            match record {
                Record::File(size) => dir_tot += size,
                Record::Directory(dir) => dir_tot += dir.borrow().tot_size,
            }
        }
        cur_dir.borrow_mut().tot_size = dir_tot;
        dir_sizes.push(dir_tot);
        if cur_dir.borrow().parent.is_some() {
            let parent = cur_dir.borrow().parent.as_ref().unwrap().upgrade().unwrap();
            cur_dir = parent;
        } else {
            break;
        }
    }

    let delete_size = needed_unused - (total_disk_space - root.borrow().tot_size);
    let mut possible_deletes: Vec<usize> =
        dir_sizes.into_iter().filter(|s| *s > delete_size).collect();
    possible_deletes.sort_unstable();

    possible_deletes[0]
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "$ cd /",
        "$ ls",
        "dir a",
        "14848514 b.txt",
        "8504156 c.dat",
        "dir d",
        "$ cd a",
        "$ ls",
        "dir e",
        "29116 f",
        "2557 g",
        "62596 h.lst",
        "$ cd e",
        "$ ls",
        "584 i",
        "$ cd ..",
        "$ cd ..",
        "$ cd d",
        "$ ls",
        "4060174 j",
        "8033020 d.log",
        "5626152 d.ext",
        "7214296 k",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 95437)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 24933642)
    }
}
