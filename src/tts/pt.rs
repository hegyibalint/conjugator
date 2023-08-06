use std::path::PathBuf;

use aws_sdk_polly::Client;
use aws_sdk_polly::types::{Engine, OutputFormat, VoiceId};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::cache::Cache;
use crate::lang::pt::PtVerbConjugations;

pub struct PtTTSProcessor<'a> {
    polly_client: &'a Client,
    cache: &'a Cache,
    sample_dir: PathBuf,
}

impl<'a> PtTTSProcessor<'a> {
    pub fn new(sample_dir: PathBuf, polly_client: &'a Client, cache: &'a Cache) -> Self {
        Self {
            sample_dir,
            polly_client,
            cache
        }
    }

    pub async fn generate_sample(&self, verbs: PtVerbConjugations) {
        let response = self.polly_client.synthesize_speech()
            .output_format(OutputFormat::OggVorbis)
            .voice_id(VoiceId::Ricardo)
            .engine(Engine::Neural)
            .text(assemble_text(verbs))
            .send()
            .await
            .unwrap();

        let mut file = File::open(self.sample_dir.join("test.ogg")).await.unwrap();
        let mut stream = response.audio_stream.into_async_read();

        let mut buffer = [0u8; 8192];
        loop {
            match stream.read(&mut buffer).await.unwrap() {
                0 => break,
                n => {
                    file.write_all(&buffer[..n]).await.unwrap();
                }
            }
        }
    }
}

fn assemble_text(verb: PtVerbConjugations) -> String {
    let mut text = String::new();
    // Add the infinitive with a dot at the end
    text.push_str(&format!("{}.", &verb.vocabulary.pt));
    // Add each conjugation combining the personal pronoun and the conjugation with a dot at the end
    for (tense, conjugation) in verb.conjugations {
        text.push_str(
            &format!(
                "{} {}.\n",
                tense.to_string(),
                conjugation.to_string()
            )
        );
    }

    text
}