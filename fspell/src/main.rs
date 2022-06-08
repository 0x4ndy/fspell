use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let spell_files =
        fs::read_dir("../spells/").expect("Something went wrong while reading directory.");

    for file in spell_files {
        println!("{:?}", file);
        if let Ok(lines) = read_lines(file.unwrap().path()) {
            for line in lines {
                if let Ok(line) = line {
                    println!("{}", line);
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
