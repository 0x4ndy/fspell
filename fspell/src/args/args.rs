use clap::{App, Arg};
use std::error::Error;

pub struct Args {
    pub config_file: String,
    pub category_name: String,
    pub subcategory_name: String,
    pub tool_name: String,
    pub search_string: String,
    is_config_file_set: bool,
    is_category_name_set: bool,
    is_subcategory_name_set: bool,
    is_tool_name_set: bool,
    is_search_string_set: bool,
}

impl Args {
    pub fn is_config_file_set(&self) -> bool {
        self.is_config_file_set
    }

    pub fn is_category_name_set(&self) -> bool {
        self.is_category_name_set
    }

    pub fn is_subcategory_name_set(&self) -> bool {
        self.is_category_name_set
    }

    pub fn is_tool_name_set(&self) -> bool {
        self.is_tool_name_set
    }

    pub fn is_search_string_set(&self) -> bool {
        self.is_search_string_set
    }

    pub fn new() -> Result<Args, Box<dyn Error>> {
        let app = Args::create_app();

        let matches = app.get_matches();

        Ok(Args {
            config_file: String::from(matches.value_of("config_file").unwrap_or("")),
            is_config_file_set: matches.is_present("config_file"),
            category_name: String::from(matches.value_of("category_name").unwrap_or("")),
            is_category_name_set: matches.is_present("category_name"),
            subcategory_name: String::from(matches.value_of("subcategory_name").unwrap_or("")),
            is_subcategory_name_set: matches.is_present("subcategory_name"),
            tool_name: String::from(matches.value_of("tool_name").unwrap_or("")),
            is_tool_name_set: matches.is_present("tool_name"),
            search_string: String::from(matches.value_of("search_string").unwrap_or("")),
            is_search_string_set: matches.is_present("search_string"),
        })
    }

    pub fn print_help() {
        Args::create_app().print_help().unwrap();
        println!();
    }

    fn create_app() -> App<'static> {
        let version = env!("CARGO_PKG_VERSION");
        let authors = env!("CARGO_PKG_AUTHORS");

        App::new("Favorite Spells")
            .version(version)
            .author(authors)
            .about("https://github.com/0x4ndy/fspell")
            .arg(
                Arg::with_name("config_file")
                    .required(false)
                    .short('c')
                    .long("config")
                    .takes_value(true)
                    .help("Configuration file."),
            )
            .arg(
                Arg::with_name("category_name")
                    .required(false)
                    .short('a')
                    .long("category")
                    .takes_value(true)
                    .help("Category name of the spell."),
            )
            .arg(
                Arg::with_name("subcategory_name")
                    .required(false)
                    .short('b')
                    .long("subcategory")
                    .takes_value(true)
                    .help("Sub-category name of the spell."),
            )
            .arg(
                Arg::with_name("tool_name")
                    .required(false)
                    .short('t')
                    .long("tool")
                    .takes_value(true)
                    .help("Tool name of the spell."),
            )
            .arg(
                Arg::with_name("search_string")
                    .required(false)
                    .short('s')
                    .long("search")
                    .takes_value(true)
                    .help("Search string."),
            )
    }
}
