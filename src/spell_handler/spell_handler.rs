use std::fs::{self, DirEntry, File, ReadDir};
use std::io::{self, BufRead};
use std::path::Path;

use crossterm::style::Stylize;

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
    pub fn from_config(config: &Config) -> Result<SpellHandler, io::Error> {
        let mut spell_files = SpellHandler::read_files(config)?;

        let spell_list: Vec<Spell> = SpellHandler::files_to_spells(&mut spell_files)?;

        Ok(SpellHandler {
            _spell_list: spell_list,
        })
    }

    pub fn get_spells(&self) -> Vec<Spell> {
        self._spell_list.clone()
    }

    pub fn search_spell<'a>(
        &'a self,
        search_parameters: &'a SearchParameters,
    ) -> impl Iterator<Item = &'a Spell> {
        self._spell_list
            .iter()
            .filter(|spell| {
                search_parameters.category.is_empty()
                    || *spell.category.to_lowercase() == search_parameters.category
            })
            .filter(|spell| {
                search_parameters.sub_category.is_empty()
                    || *spell.sub_category.to_lowercase() == search_parameters.sub_category
            })
            .filter(|spell| {
                search_parameters.tool.is_empty()
                    || *spell.tool.to_lowercase() == search_parameters.tool
            })
            .filter(move |spell| {
                search_parameters.search_str.is_empty()
                    || spell
                        .title
                        .to_lowercase()
                        .contains(&search_parameters.search_str)
            })
    }

    fn lines_to_spells(file: &DirEntry, lines: io::Lines<io::BufReader<File>>) -> Vec<Spell> {
        let mut spell_list: Vec<Spell> = Vec::new();
        let current_category = String::from(file.path().file_stem().unwrap().to_str().unwrap());

        let mut in_spell = false;
        let mut current_sub_category = String::from("");
        let mut current_title = String::from("");
        let mut current_tool = String::from("");
        let mut current_code = String::from("");

        for (i, line) in lines.enumerate() {
            if let Ok(line) = line {
                if line.starts_with(STR_SUBCATEGORY_MARKER) && !in_spell {
                    match line.as_str().split_once(" ") {
                        None => panic!(
                            "Line {} is marked as category name, but it doesn't contain any value.",
                            i
                        ),
                        Some((_, sub_cat)) => current_sub_category = String::from(sub_cat),
                    }
                } else if line.starts_with(STR_TOOL_MARKER) {
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
                } else if line == STR_SPELL_END_MARKER {
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
                } else if !line.starts_with(STR_CODE_MARKER) && in_spell {
                    current_code.push_str(format!("{}\n", line).as_str());
                }
            }
        }

        spell_list
    }

    fn files_to_spells(dir_content: &mut ReadDir) -> Result<Vec<Spell>, io::Error> {
        let mut spell_list: Vec<Spell> = Vec::new();

        for dir_entry in dir_content {
            let file = dir_entry?;
            let file_path = file.path();
            let lines = SpellHandler::read_lines(file_path)?;
            let mut spells = SpellHandler::lines_to_spells(&file, lines);

            spell_list.append(&mut spells);
        }

        Ok(spell_list)
    }

    fn read_files(config: &Config) -> Result<ReadDir, std::io::Error> {
        println!(
            "{} {}",
            "Reading spells from:".green(),
            config.spells_dir.clone().bold().underlined()
        );

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

const STR_SUBCATEGORY_MARKER: &str = "# ";
const STR_TOOL_MARKER: &str = "### ";
const STR_CODE_MARKER: &str = "```";
const STR_SPELL_END_MARKER: &str = "---";
