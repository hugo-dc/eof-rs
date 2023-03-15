use std::io;

use std::fs::File;
use std::io::BufReader;
//use std::path::PathBuf;

use eof_rs::*;
use hex::FromHex;
//use eof_rs::error

//use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, ArgMatches, Command};
//use clap::{arg, command, value_parser, ArgAction, Command};
use clap::{arg, command, Command};

fn validate(input: Option<&String>) -> Result<()> {
    let reader: std::result::Result<EOFContainer, serde_json::Error> = if let Some(path) = input {
        serde_json::from_reader(BufReader::new(File::open(path)?))
    } else {
        serde_json::from_reader(io::stdin())
    };
    reader?.is_valid_eof()
}

fn convert(input: Option<&String>, fmt: &str) -> Result<()> {
    if input.is_some() {
        let a = Vec::from_hex(input.unwrap()).unwrap();
        let container = eof_rs::from_slice(&a).unwrap(); // FIXME: translate error
        
        container.is_valid_eof()?;

        if fmt == "json" {
            let json = serde_json::to_string(&container).unwrap();
            println!("{}", json)
        } else {
            unimplemented!();
        }
        return Ok(());

    } else {
        panic!("invalid input");
    }
}

fn main() -> Result<()> {
    let matches = command!()
        .subcommand_required(true)
        .subcommand(
        Command::new("validate").about("validates a given EOF structure")
        .arg(arg!([input] "Input file to operate on (stdin if omitted)"))
        )
        .subcommand(
            Command::new("convert")
                .about("converts between various representations")
        .arg(arg!([input] "Input file to operate on (stdin if omitted)"))
                .arg(
                    arg!(--fmt <FMT> "target format (bin, json, yaml)").required(true), //.value_parser(value_parser!(PathBuf)),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("validate") {
        validate(matches.get_one::<String>("input"))?
    } else if let Some(matches) = matches.subcommand_matches("convert") {
        let fmt = matches.get_one::<String>("fmt").expect("ensurde by clap");
        convert(matches.get_one::<String>("input"), fmt)?
    }

    Ok(())
}

