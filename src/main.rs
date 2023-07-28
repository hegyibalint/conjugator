use std::path::Path;
use std::str::FromStr;
use clap::Parser;
use serde::de::DeserializeOwned;
use crate::lang::pt::PortugueseLangProcessor;
use crate::lang::{LangProcessor};

mod lang;
mod scraper;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    language: Language,
}

#[derive(Debug, Clone)]
enum Language {
    Portuguese,
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "pt" => Ok(Language::Portuguese),
            lang => Err(format!("Unknown language: '{}'", lang).into()),
        }
    }
}

fn process<T>(filename: &str, process: impl Fn(T)) where T: DeserializeOwned {
    // If the filename is not found, this will panic.
    if !Path::new(filename).exists() {
        panic!("File not found: {}", filename);
    }

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)
        .unwrap();
    for record in rdr.deserialize() {
        let record: T = record.unwrap();
        process(record);
    }
}

fn main() {
    let args = Args::parse();

    match args.language {
        Language::Portuguese => {
            let processor = PortugueseLangProcessor::new();
            process("verbs.csv", |verb| processor.process_verb(verb));
        }
    }
}



