// Day 07 No Space Left on Device

use std::collections::{HashMap, HashSet};
use std::hash::{Hash};
use std::io::{BufRead, BufReader};
use std::path::Path;
use regex::Regex;
use regex::RegexSet;
use lazy_static::lazy_static;


lazy_static! {
    static ref CD_RE: Regex = Regex::new(r"^\$ cd (.+)$").unwrap();
    static ref LS_RE: Regex = Regex::new(r"^\$ ls\s*$").unwrap();
    static ref DIR_RE: Regex = Regex::new(r"^dir (.+)$").unwrap();
    static ref FILE_RE: Regex = Regex::new(r"^(\d+) (.+)$").unwrap();
    static ref RE_SET: RegexSet = RegexSet::new(&[
        r"^\$ cd (.+)$",
        r"^\$ ls\s*$",
        r"^dir (.+)$",
        r"^(\d+) (.+)$"
    ]).unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FileInfo {
    name: String,
    size: usize,
}

impl FileInfo {
    fn new(name: String, size: usize) -> Self {
        Self {
            name, size
        }
    }
}

#[derive(Debug, Clone)]
struct FileNode {
    parent: String,
    size: usize,
    files: HashSet<FileInfo>,
    dirs: HashSet<FileInfo>,
}

impl FileNode {
    fn new(parent: String, size: usize, files: HashSet<FileInfo>, dirs: HashSet<FileInfo>) -> Self {
        Self {
            parent, size, files, dirs
        }
    }
}

type FileMap = HashMap<String, FileNode>;

/// parse the input data into a HashMap
fn parse_to_map(path: &Path) -> FileMap {
    let f = std::fs::File::open(path).unwrap();
    let reader = BufReader::new(f);

    // file map maps the absolute path of a file to a FileNode
    let mut file_map: HashMap<String, FileNode> = HashMap::new();
    // cur_path is the current directory path
    let mut cur_path: Vec<String> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let matches: Vec<_> = RE_SET.matches(&line).into_iter().collect();
        match &matches[..] {
            [0] => {
                let dir_name = CD_RE.captures(&line).unwrap()[1].to_string();
                match dir_name.as_str() {
                    ".." => {
                        cur_path.pop();
                    }
                    "/" => {
                        cur_path.clear();
                        cur_path.push("".to_string());
                    },
                    _ => {
                        cur_path.push(dir_name);
                    }
                }
            },
            [1] => {},
            [2] => {
                let dir_name = DIR_RE.captures(&line).unwrap()[1].to_string();
                let mut parent_dir = cur_path.join("/");
                if parent_dir.is_empty() { parent_dir = String::from("/"); }
                // get file node if it exists, and update its vec of directories
                let node = file_map.entry(parent_dir.clone()).or_insert(FileNode::new(parent_dir.clone(), 0, HashSet::new(), HashSet::new()));
                node.dirs.insert(FileInfo::new(dir_name, 0));
            },
            [3] => {
                let file_size = FILE_RE.captures(&line).unwrap()[1].to_string().parse::<usize>().unwrap();
                let file_name = FILE_RE.captures(&line).unwrap()[2].to_string();
                let mut parent_dir = cur_path.join("/");
                if parent_dir.is_empty() { parent_dir = String::from("/"); }
                // get file node if it exists, and update its vec of files
                let node = file_map.entry(parent_dir.clone()).or_insert(FileNode::new(parent_dir.clone(), 0, HashSet::new(), HashSet::new()));
                node.files.insert(FileInfo::new(file_name, file_size));
            },
            _ => {
                panic!("UNKNOWN MATCH {}", &line)
            }
        }

    }
    file_map
}

fn compute_sizes(fmap: &mut FileMap) {
    // find path of leaf directories (directories that contain only files)
    let mut leaf_nodes = fmap.iter()
        .filter(|&(_path, node)| node.dirs.is_empty())
        .map(|(path, _node)| path)
        .cloned()
        .collect::<Vec<String>>();

    // sort leaf nodes by their depth
    leaf_nodes.sort_by(|n1, n2| {
        let n1_level = n1.split("/").filter(|s| !s.is_empty()).collect::<Vec<_>>().len();
        let n2_level = n2.split("/").filter(|s| !s.is_empty()).collect::<Vec<_>>().len();
        n1_level.cmp(&n2_level)
    });

    // keeps track of paths that have been visited
    let mut visited: HashSet<String> = HashSet::new();

    // iterate through the leaf_nodes and their parent dirs, computing the file and dir sizes as we go
    while let Some(leaf_node) = leaf_nodes.pop() {
        let mut paths = leaf_node.split("/").collect::<Vec<&str>>();

        while !paths.is_empty() {
            let mut cur_path = paths.join("/");
            if cur_path.is_empty() { cur_path = "/".to_string(); }

            if !visited.contains(&cur_path) {
                // compute size of files at the current path
                let file_size = fmap.get(&cur_path)
                    .expect("fileNode exists at path")
                    .files.iter()
                    .map(|f| f.size)
                    .sum::<usize>();
                fmap.get_mut(&cur_path).unwrap().size += file_size;

                // get paths to child directories
                let child_paths = fmap.get(&cur_path)
                    .unwrap()
                    .dirs
                    .iter()
                    .map(|fd| {
                        if cur_path == "/" {
                            format!("{}{}", &cur_path, fd.name)
                        } else {
                            format!("{}/{}", &cur_path, fd.name)
                        }
                    })
                    .collect::<Vec<String>>();

                // update sizes of dirs in the current path
                let total_child_size = child_paths
                    .into_iter()
                    .map(|s| fmap.get(&s).unwrap().size)
                    .sum::<usize>();

                fmap.get_mut(&cur_path).unwrap().size += total_child_size;
                visited.insert(cur_path.clone());
            }

            paths.pop();
        }

    }
}



fn part1() {
    let path = Path::new("../input/d07-input.txt");
    let mut map = parse_to_map(path);
    compute_sizes(&mut map);

    // find all dirs with a total size of at most 100_000
    let mut total = 0_usize;
    for (k, v) in map.iter().filter(|&(_p, n)| n.size <= 100_000) {
        println!("{}  {:?}", k, v);
        total += v.size;
    }
    // find the total sum of those dirs
    println!("part 1 total sum of dirs {}", total);
}

/// find the directory closest to 30_000_000
/// ans = 24_933_642
fn part2() {
    let path = Path::new("../input/d07-input.txt");
    let mut map = parse_to_map(path);
    compute_sizes(&mut map);

    // for (k, v) in map.iter() {
    //     println!("{} : {:?}", k, v);
    // }

    // tot_used =  3_693_945  real used= 48_381_165
    // tot_un   = 66_306_055
    // target   = 36_306_055
    let used_space = map.get("/").unwrap().size;
    println!("total used at root {}", &used_space);
    let unused_space = 70_000_000 - used_space;
    println!("total unused {}", &unused_space);
    let target = unused_space - 30_000_000;
    println!("total need space to reach 30_000_000 {}", &target);

    // sort by FileNode.size and take the first FileNode
    let mut nodes: Vec<&FileNode> = map.values().filter(|n| n.size >= target).collect::<Vec<&FileNode>>();
    nodes.sort_by_key(|n| n.size);
    for node in nodes.iter() {
        println!("{}", &node.size);
    }
}


#[cfg(test)]
mod tests {
    use crate::d07_no_space_left::{part1, part2};

    #[test]
    fn test_part1() {
        part1();
    }

    #[test]
    fn test_part2() {
        part2();
    }

    #[test]
    fn test_join() {
        let v: Vec<&str> = vec![""];
        let j = &v.join("/");
        println!(":{}:", &j);
    }

    #[test]
    fn test_split() {
        let s1 = "/twjcmp/fpp";
        let sp1 = s1.split("/").collect::<Vec<&str>>();
        dbg!(&sp1);
        dbg!(sp1.join("/"));
    }
}