pub mod pt;

#[derive(Debug, serde::Deserialize)]
pub struct VocabularyVerb {
    source_lang: String,
    target_lang: String,
    #[serde(default)]
    comments: String
}

#[derive(Debug, serde::Deserialize)]
pub struct VocabularyNoun {
    src_language: String,
    target_language: String,
    comments: String
}

pub trait LangProcessor {
    fn process_verb(&self, verb: VocabularyVerb);
    fn process_noun(&self, noun: VocabularyNoun);
}

pub struct Verb<C> {
    s_1: ConjugatedVerb<C>,
    s_2: ConjugatedVerb<C>,
    s_3: ConjugatedVerb<C>,
    p_1: ConjugatedVerb<C>,
    p_2: ConjugatedVerb<C>,
    p_3: ConjugatedVerb<C>,
}

pub enum ConjugatedVerb<V> {
    REGULAR(V),
    IRREGULAR(V),
}