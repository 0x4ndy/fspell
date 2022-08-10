# Favourite Spells
This repository contains a list of my favourite and most used spells (commands and arguments), separated into a number of categories categories. See the list of categories below.

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

## Table of Contents
### [Fuzzing](https://github.com/0x4ndy/fspell/blob/master/spells/fuzzing.md)
- [Web Files](https://github.com/0x4ndy/fspell/blob/master/spells/fuzzing.md#Web_Files)
- [Web Directories](https://github.com/0x4ndy/fspell/blob/master/spells/fuzzing.md#Web_Directories)
- [Sub-domains](https://github.com/0x4ndy/fspell/blob/master/spells/fuzzing.md#Sub-domains)
- [VHosts](https://github.com/0x4ndy/fspell/blob/master/spells/fuzzing.md#VHosts)
- [Parameters](https://github.com/0x4ndy/fspell/blob/master/spells/fuzzing.md#Parameters)
### [Recon](https://github.com/0x4ndy/fspell/blob/master/spells/recon.md)
- [DNS](https://github.com/0x4ndy/fspell/blob/master/spells/recon.md#DNS)
- [Ports](https://github.com/0x4ndy/fspell/blob/master/spells/recon.md#Ports)
- [SMB](https://github.com/0x4ndy/fspell/blob/master/spells/recon.md#SMB)
- [SMTP](https://github.com/0x4ndy/fspell/blob/master/spells/recon.md#SMTP)
- [SNMP](https://github.com/0x4ndy/fspell/blob/master/spells/recon.md#SNMP)
- [VULNs](https://github.com/0x4ndy/fspell/blob/master/spells/recon.md#VULNs)
### [Transfers](https://github.com/0x4ndy/fspell/blob/master/spells/transfers.md)
- [NSF](https://github.com/0x4ndy/fspell/blob/master/spells/transfers.md#NFS)
### [XSS](https://github.com/0x4ndy/fspell/blob/master/spells/xss.md)
- [Payloads](https://github.com/0x4ndy/fspell/blob/master/spells/xss.md#Payloads)
