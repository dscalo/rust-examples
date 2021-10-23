use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::any::type_name;

struct Edge {
    to: u32,
    dist: u32,
}

struct Node {
    id: u32,
    edges: Vec<Edge>,
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("files/test01.txt") {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    } else {
        panic!("File not found.");
    }
}
