use std::io;

use std::fs::File;
use std::io::BufReader;
use eof_rs::*;
use hex::FromHex;

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
    let code: Vec<u8>;
    if input.is_some() {
        code = Vec::from_hex(input.unwrap()).unwrap();
    } else {
        code = Vec::from_hex(io::read_to_string(io::stdin()).unwrap().trim()).unwrap();
    }

    let container = eof_rs::from_slice(&code)?;
    container.is_valid_eof()?;

    if fmt == "json" {
        let json = serde_json::to_string(&container).unwrap();
        println!("{}", json)
    } else {
        unimplemented!();
    }
    return Ok(());
}

fn main() -> Result<()> {
    let matches = command!()
        .subcommand_required(true)
        .subcommand(
            Command::new("validate")
                .about("validates a given EOF structure")
                .arg(arg!([input] "Input file to operate on (stdin if omitted)")),
        )
        .subcommand(
            Command::new("convert")
                .about("converts between various representations")
                .arg(arg!([input] "Input file to operate on (stdin if omitted)"))
                .arg(
                    arg!(--fmt <FMT> "target format (bin, json, yaml)").required(true),
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
