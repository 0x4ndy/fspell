use std::fs::{self, DirEntry, File, ReadDir};
use std::io::{self, BufRead};
use std::path::Path;

use crate::config::Config;

#[derive(Clone)]
pub struct Spell {
    pub title: String,
    pub code: String,
    pub category: String,
    pub sub_category: String,
    pub tool: String,
    pub file_name: String,
}

pub struct SpellHandler {
    _spell_list: Vec<Spell>,
}

#[derive(Debug)]
pub struct SearchParameters {
    pub category: String,
    pub sub_category: String,
    pub tool: String,
    pub search_str: String,
}

impl SpellHandler {
    pub fn from_config(config: Config) -> Result<SpellHandler, io::Error> {
        let spell_files = SpellHandler::read_files(config)?;

        let spell_list: Vec<Spell> = SpellHandler::files_to_spells(spell_files)?;

        Ok(SpellHandler {
            _spell_list: spell_list,
        })
    }

    pub fn get_spells(&self) -> Vec<Spell> {
        self._spell_list.clone()
    }

    pub fn search_spell(&self, search_parameters: SearchParameters) -> Vec<Spell> {
        let tmp_results: Vec<Spell> = self
            ._spell_list
            .clone()
            .into_iter()
            .filter(|spell| {
                if search_parameters.category.is_empty() {
                    true
                } else {
                    *spell.category == search_parameters.category
                }
            })
            .filter(|spell| {
                if search_parameters.sub_category.is_empty() {
                    true
                } else {
                    *spell.sub_category == search_parameters.sub_category
                }
            })
            .filter(|spell| {
                if search_parameters.tool.is_empty() {
                    true
                } else {
                    *spell.tool == search_parameters.tool
                }
            })
            .filter(|spell| {
                if search_parameters.search_str.is_empty() {
                    true
                } else {
                    //*spell.search_str.contains(search_parameters.search_str)
                    true
                }
            })
            .collect();

        // TODO: replace this part by including in the filer above  
        let mut results: Vec<Spell> = Vec::new();
        for spell in tmp_results {
            if search_parameters.search_str.is_empty() {
                results.push(spell);
            } else {
                if spell
                    .title
                    .to_lowercase()
                    .contains(&search_parameters.search_str.to_lowercase())
                {
                    results.push(spell);
                }
            }
        }

        results
    }

    fn lines_to_spells(file: DirEntry, lines: io::Lines<io::BufReader<File>>) -> Vec<Spell> {
        let mut spell_list: Vec<Spell> = Vec::new();
        let current_category = String::from(file.path().file_stem().unwrap().to_str().unwrap());

        let mut in_spell = false;
        let mut current_sub_category = String::from("");
        let mut current_title = String::from("");
        let mut current_tool = String::from("");
        let mut current_code = String::from("");

        for (i, line) in lines.enumerate() {
            if let Ok(line) = line {
                if line.starts_with("# ") {
                    match line.as_str().split_once(" ") {
                        None => panic!(
                            "Line {} is marked as category name, but it doesn't contain any value.",
                            i
                        ),
                        Some((_, sub_cat)) => current_sub_category = String::from(sub_cat),
                    }
                } else if line.starts_with("### ") {
                    match line.as_str().split_once(" ") {
                        None => panic!(
                            "Line {} is marked as spell title, but it doesn't contain any value.",
                            i
                        ),
                        Some((_, title)) => {
                            match title.split_once(":") {
                                None => current_title = String::from(title),
                                Some((tool, title)) => {
                                    current_title = String::from(title);
                                    current_tool = String::from(tool);
                                }
                            }

                            in_spell = true;
                        }
                    }
                } else if line == "---" {
                    let spell = Spell {
                        title: String::from(&current_title),
                        code: String::from(&current_code),
                        category: String::from(&current_category),
                        sub_category: String::from(&current_sub_category),
                        tool: String::from(&current_tool),
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

    fn files_to_spells(dir_content: ReadDir) -> Result<Vec<Spell>, io::Error> {
        let mut spell_list: Vec<Spell> = Vec::new();

        for dir_entry in dir_content {
            let file = dir_entry?;
            let file_path = file.path();
            let lines = SpellHandler::read_lines(file_path)?;
            let mut spells = SpellHandler::lines_to_spells(file, lines);

            spell_list.append(&mut spells);
        }

        Ok(spell_list)
    }

    fn read_files(config: Config) -> Result<ReadDir, std::io::Error> {
        println!("Reading spells from: {}", config.spells_dir);

        match fs::read_dir(&config.spells_dir) {
            Ok(v) => Ok(v),
            Err(e) => {
                return Err(io::Error::new(
                    e.kind(),
                    format!("{}: {}", config.spells_dir, e.to_string()),
                ));
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
}
