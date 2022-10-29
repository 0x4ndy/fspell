# fspell
`fspell` is a CLI tool allowing for an easy browsing and searching of your favourite spells without leaving your terminal.

## Set up
Checkout the GitHub repository:
```bash
git clone https://github.com/0x4ndy/fspell.git <project-directory>
```
The source code is located under ``<project-directory>/fspell`` directory

## Usage
```bash
fspell --help
```
```bash
Favorite Spells 0.2.0

https://github.com/0x4ndy/fspell

USAGE:
    fspell [OPTIONS]

    OPTIONS:
        -a, --category <category_name>          Category name of the spell.
        -b, --subcategory <subcategory_name>    Sub-category name of the spell.
        -c, --config <config_file>              Configuration file.
        -h, --help                              Print help information
        -s, --search <search_string>            Search string.
        -t, --tool <tool_name>                  Tool name of the spell.
        -V, --version                           Print version information
```
Example:
```bash
fspell -a recon -b dns
```
