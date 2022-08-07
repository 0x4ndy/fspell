mod args;
mod config;
mod spell_handler;

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

    let mut config_file = Config::get_config_path();
    if args.is_config_file_set() {
        config_file = args.config_file;
    }

    println!("Using config: {}", config_file);

    let search_parameters: SearchParameters = SearchParameters {
        category: args.category_name,
        sub_category: args.subcategory_name,
        tool: args.tool_name,
        search_str: args.search_string,
    };

    println!("{:?}", search_parameters);

    let config = Config::from(config_file.as_str())?;

    let spell_handler = SpellHandler::from_config(config)?;

    let spell_list = spell_handler.search_spell(&search_parameters);

    for spell in spell_list {
        println!(
            "{}: {} - {} (using: {})\nFile: {}\n{}",
            spell.category,
            spell.sub_category,
            spell.title,
            spell.tool,
            spell.file_name,
            spell.code
        );
    }

    Ok(())
}
