use std::path::PathBuf;
use std::str::FromStr;

use aws_sdk_polly::Client;
use aws_sdk_polly::config::Region;
use clap::Parser;
use url::Url;

use crate::anki::AnkiHandler;
use crate::csv::read_csv;
use crate::lang::pt::conjugate;
use crate::tts::pt::PtTTSProcessor;

mod lang;
mod anki;
mod tts;
mod csv;

const DEFAULT_ANKI_SERVER: &str = "http://127.0.0.1:8080";
const DEFAULT_MEDIA_DIR: &str = "anki/media";

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    language: Language,

    #[arg(short, long)]
    verbs_csv: Option<PathBuf>,
    #[arg(short, long)]
    nouns_csv: Option<PathBuf>,
    #[arg(short, long)]
    adjectives_csv: Option<PathBuf>,
    #[arg(short, long)]
    etc_csv: Option<PathBuf>,

    #[arg(long)]
    cache_dir: Option<PathBuf>,

    #[arg(long)]
    aws_region: String,
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
            lang => Err(format!("Unknown language: '{}'", lang)),
        }
    }
}

impl Into<String> for Language {
    fn into(self) -> String {
        match self {
            Language::Portuguese => "pt".to_string(),
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let cache_path = args.cache_dir.unwrap_or(PathBuf::from(".cache"));
    let verbs_csv = args.verbs_csv.unwrap_or(PathBuf::from("verbs.csv"));

    let aws_region = Region::new(args.aws_region);
    let aws_config = aws_config::from_env().region(aws_region).load().await;
    let polly_client = Client::new(&aws_config);

    let mut anki_handler = AnkiHandler::new(
        PathBuf::from(DEFAULT_MEDIA_DIR),
        Url::parse(DEFAULT_ANKI_SERVER).unwrap(),
    );
    anki_handler.prepare();

    match &args.language {
        Language::Portuguese => process_pt(verbs_csv, &polly_client, &mut anki_handler, cache_path).await,
    }
}

async fn process_pt(verbs_csv: PathBuf, polly_client: &Client, anki_handler: &mut AnkiHandler, cache_dir: PathBuf) {
    // let tts = PtTTSProcessor::new(
    //     PathBuf::from(DEFAULT_MEDIA_DIR),
    //     polly_client,
    //     cache
    // );

    for verb in read_csv(verbs_csv, csv::pt::process_verb) {
        if !anki_handler.has_verb(&verb) {
            println!("Adding verb '{}'", &verb.pt);
            let conj_verb = conjugate(&verb, cache_dir).await;
            anki_handler.add_verb(&conj_verb);
        } else {
            println!("Verb '{}' already exists", &verb.pt);
        }


        // tts.generate_sample(conj_verb).await;
    };
}

