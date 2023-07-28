use crate::lang::{ConjugatedVerb, VocabularyVerb};
use crate::lang::pt::PortugueseVerb;

pub mod pt;

trait VerbScraper<V>  {
    fn scrape(vocab_verb: VocabularyVerb) -> ConjugatedVerb<V>;
}