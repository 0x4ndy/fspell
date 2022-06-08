use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

struct Spell {
    title: String,
    code: String,
    category: String,
}

fn main() {
    let spell_files =
        fs::read_dir("../spells/").expect("Something went wrong while reading directory.");

    let spell_list: Vec<Spell> = Vec::new();

    for file in spell_files {
        if let Ok(lines) = read_lines(file.unwrap().path()) {
            let new_spell: bool = false;
            let current_category: String = String::from("");

            for line in lines {
                if let Ok(line) = line {
                    if line.starts_with("# ") {
                        // split and assign the value
                    }
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
