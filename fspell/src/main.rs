use std::fs::{self, File, ReadDir};
use std::io::{self, BufRead};
use std::path::Path;

struct Spell {
    title: String,
    code: String,
    category: String,
    sub_category: String,
}

fn main() {
    let spell_files =
        fs::read_dir("../spells/").expect("Something went wrong while reading directory.");

    let spell_list: Vec<Spell> = files_to_spells(spell_files);

    for spell in &spell_list {
        println!("{}:{} - {}\n{}", spell.category, spell.sub_category, spell.title, spell.code);
    }
}

fn files_to_spells(dir_content: ReadDir) -> Vec<Spell> {
    let mut spell_list: Vec<Spell> = Vec::new();
     
    for file in dir_content {
        if let Ok(lines) = read_lines(file.as_ref().unwrap().path()) {
            let current_category = String::from(
                file.unwrap()
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
                            Some((mark, sub_cat)) => current_sub_category = String::from(sub_cat),
                        }
                    } else if line.starts_with("### ") {
                        match line.as_str().split_once(" ") {
                            None => panic!("Line {} is marked as spell title, but it doesn't contain any value.", i),
                            Some((mark, title)) => {
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
                        };

                        spell_list.push(spell); 

                        in_spell = false;
                        current_code = String::from("");
                    } else if !line.starts_with("```") && in_spell {
                        current_code.push_str(format!("{}\n", line).as_str());
                    }
                }
            }
        }

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
