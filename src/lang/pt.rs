use crate::lang::{LangProcessor, VocabularyNoun, VocabularyVerb};

pub struct PortugueseLangProcessor;

pub struct PortugueseVerb {
    root: String,
    suffix: String,
}

impl PortugueseLangProcessor {
    pub fn new() -> Self {
        Self
    }
}

impl LangProcessor for PortugueseLangProcessor {
    fn process_verb(&self, verb: VocabularyVerb) {

    }

    fn process_noun(&self, noun: VocabularyNoun) {
        todo!()
    }
}