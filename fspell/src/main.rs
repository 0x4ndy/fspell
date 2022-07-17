use std::fs::{self, File, ReadDir, DirEntry};
use std::io::{self, BufRead};
use std::path::Path;

struct Spell {
    title: String,
    code: String,
    category: String,
    sub_category: String,
    file_name: String,
}

fn main() {
    let spell_files =
        fs::read_dir("../spells/").expect("Something went wrong while reading directory.");

    let spell_list: Vec<Spell> = files_to_spells(spell_files);

    for spell in &spell_list {
        println!("{}: {} - {}\nFile: {}\n{}", spell.category, spell.sub_category, spell.title, spell.file_name, spell.code);
    }
}

fn lines_to_spells(file: DirEntry, lines: io::Lines<io::BufReader<File>>) -> Vec<Spell> {
    let mut spell_list: Vec<Spell> = Vec::new();
    let current_category = String::from(
        file
        .path()
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap());

    let mut in_spell = false;
    let mut current_sub_category = String::from("");
    let mut current_title = String::from("");
    let mut current_code = String::from("");

    for (i, line) in lines.enumerate() {
        if let Ok(line) = line {
            if line.starts_with("# ") {
                match line.as_str().split_once(" ") {
                    None => panic!("Line {} is marked as category name, but it doesn't contain any value.", i),
                    Some((_, sub_cat)) => current_sub_category = String::from(sub_cat),
                }
            } else if line.starts_with("### ") {
                match line.as_str().split_once(" ") {
                    None => panic!("Line {} is marked as spell title, but it doesn't contain any value.", i),
                    Some((_, title)) => {
                        current_title = String::from(title);
                        in_spell = true;
                    },
                }
            } else if line == "---" {
                let spell = Spell {
                   title: String::from(&current_title),
                   code: String::from(&current_code),
                   category: String::from(&current_category),
                   sub_category: String::from(&current_sub_category),
                   file_name: String::from("file"),
                };

                spell_list.push(spell); 

                in_spell = false;
                current_code = String::from("");
            } else if !line.starts_with("```") && in_spell {
                current_code.push_str(format!("{}\n", line).as_str());
            }
        }
    }

    spell_list
}

fn files_to_spells(dir_content: ReadDir) -> Vec<Spell> {
    let mut spell_list: Vec<Spell> = Vec::new();
     
    for dir_entry in dir_content {

        let file = match dir_entry {
            Err(e) => panic!("Can't process the file: {e}"),
            Ok(file) => file,
        };

        let file_path = file.path();

        let lines = match read_lines(file_path) {
            Err(e) => panic!("{e}"),
            Ok(lines) => lines,
        };

        let mut spells = lines_to_spells(file, lines);

        spell_list.append(&mut spells);
    }

    spell_list
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
