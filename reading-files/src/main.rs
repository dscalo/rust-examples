use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::any::type_name;



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

fn create_graph (filename: &str) -> Vec<Vec<u32>> {
    let mut numbs = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
              let row =  ip.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
                numbs.push(row);
            }
        }
    }
    
    numbs

}

fn main() {
    
    let numbs = create_graph("files/test01.txt");

    for num in numbs {
        println!("{:?}", num);
    }    

}
