mod args;
mod config;
mod spell_handler;

use crossterm::style::Stylize;
use std::error::Error;

use crate::args::Args;
use crate::config::Config;
use crate::spell_handler::*;

fn main() -> Result<(), Box<dyn Error>> {
    let args = match Args::new() {
        Ok(args) => args,
        Err(e) => {
            Args::print_help();
            return Err(e);
        }
    };

    let mut config_file: Option<String> = None;
    if args.is_config_file_set() {
        config_file = Some(args.config_file);
    }


    let search_parameters: SearchParameters = SearchParameters {
        category: args.category_name,
        sub_category: args.subcategory_name,
        tool: args.tool_name,
        search_str: args.search_string,
    };

    println!("{:?}", search_parameters);

    let config = match config_file {
        None => Config::new()?,
        Some(config_file) => Config::from(config_file.as_str())?,
    };

    let spell_handler = SpellHandler::from_config(&config)?;

    let spell_list = spell_handler.search_spell(&search_parameters);

    println!();
    for spell in spell_list {
        println!(
            "{}: {} - {} (using: {})\n{}\n",
            spell.category.trim().green(),
            spell.sub_category.trim().green(),
            spell.title.trim(),
            spell.tool.trim().blue(),
            spell.code.trim().clone().on_dark_grey()
        );
    }

    Ok(())
}
