use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use scraper::{ElementRef, Selector};

// ============================================================================
// VERBS
// ============================================================================

#[derive(Debug, Clone)]
pub struct PtVocabularyVerb {
    pub en: String,
    pub pt: String,
    pub comments: Option<String>,
}

#[derive(Debug, Clone)]
pub enum PtVerbSuffix {
    Regular(String),
    Irregular(String),
}

#[derive(Debug, Clone)]
pub struct PtVerbConjugation {
    pub root: Option<String>,
    pub suffix: PtVerbSuffix,
}

impl Display for PtVerbConjugation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let root = match &self.root {
            Some(v) => v,
            None => "",
        };
        let suffix = match &self.suffix {
            PtVerbSuffix::Regular(v) => v,
            PtVerbSuffix::Irregular(v) => v,
        };
        write!(f, "{}{}", root, suffix)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PtPersonalPronoun {
    S1,
    S2,
    S3,
    P1,
    P2,
    P3,
}

impl Display for PtPersonalPronoun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let person = match self {
            PtPersonalPronoun::S1 => "Eu",
            PtPersonalPronoun::S2 => "Tu",
            PtPersonalPronoun::S3 => "Ele/Ela/Você",
            PtPersonalPronoun::P1 => "Nós",
            PtPersonalPronoun::P2 => "Vós",
            PtPersonalPronoun::P3 => "Eles/Elas/Vocês",
        };
        write!(f, "{}", person)
    }
}

#[derive(Debug, Clone)]
pub struct PtVerbConjugations {
    pub vocabulary: PtVocabularyVerb,
    pub conjugations: HashMap<PtPersonalPronoun, PtVerbConjugation>,
}

impl PtVerbConjugations {
    pub fn new(vocabulary: PtVocabularyVerb) -> Self {
        Self {
            vocabulary,
            conjugations: HashMap::new()
        }
    }
}

// Conjugator -----------------------------------------------------------------

static CONJUGATION_ENDPOINT_STUB: &str = "https://conjugator.reverso.net/conjugation-portuguese-verb-";

fn person_to_person(person: &str) -> PtPersonalPronoun {
    match person {
        "eu" => PtPersonalPronoun::S1,
        "tu" => PtPersonalPronoun::S2,
        "ele/ela/você" => PtPersonalPronoun::S3,
        "nós" => PtPersonalPronoun::P1,
        "vós" => PtPersonalPronoun::P2,
        "eles/elas/vocês" => PtPersonalPronoun::P3,
        _ => panic!("Invalid person: {}", person),
    }
}

fn process_tense(vocabulary_verb: &PtVocabularyVerb, tense_box: ElementRef) -> PtVerbConjugations {
    let li_selector = Selector::parse("ul > li").unwrap();
    let person_selector = Selector::parse("i:first-child").unwrap();
    let root_selector = Selector::parse("i:last-child > i.verbtxt").unwrap();
    let conj_reg_selector = Selector::parse("i:last-child > i.verbtxt-term").unwrap();
    let conj_ireg_selector = Selector::parse("i:last-child > i.verbtxt-term-irr").unwrap();

    let mut verb_conjugations = PtVerbConjugations::new(vocabulary_verb.clone());

    for li in tense_box.select(&li_selector) {
        let person = person_to_person(
            li.select(&person_selector).next().unwrap().inner_html().as_str()
        );
        let verb_root = li
            .select(&root_selector)
            .next()
            .map(|root| root.inner_html());
        let conj_reg = li
            .select(&conj_reg_selector)
            .next()
            .map(|conj_reg| conj_reg.inner_html());
        let conj_ireg = li
            .select(&conj_ireg_selector)
            .next()
            .map(|conj_ireg| conj_ireg.inner_html());

        if conj_reg.is_some() && conj_ireg.is_some() {
            panic!("Both regular and irregular conjugations found!");
        }
        let conj = if let Some(suffix) = conj_reg {
            PtVerbSuffix::Regular(suffix)
        } else if let Some(suffix) = conj_ireg {
            PtVerbSuffix::Irregular(suffix)
        } else {
            panic!("Neither regular nor irregular conjugations found!");
        };

        let verb = PtVerbConjugation {
            root: verb_root,
            suffix: conj,
        };

        if let Some(previous) = verb_conjugations.conjugations.insert(person.clone(), verb.clone()) {
            panic!("Duplication detected for person {}. Current verb: {}, Previous verb: {}", person, verb, previous);
        }
    }

    verb_conjugations
}

fn process_document(vocabulary_verb: &PtVocabularyVerb, document: &str) -> PtVerbConjugations {
    let query_box_selector = Selector::parse("div.result-block-api > div.word-wrap-row > div.wrap-three-col > div.blue-box-wrap").unwrap();
    let p_selector = Selector::parse("p").unwrap();

    let document = scraper::Html::parse_document(document);

    for query_box in document.select(&query_box_selector) {
        let tense_name = query_box.select(&p_selector).next().unwrap().inner_html();
        // Bit dirty right now, but we only want to process the present tense
        if tense_name != "Presente" {
            continue;
        }

        return process_tense(vocabulary_verb, query_box);
    }

    panic!("Present tense was not found!");
}

pub async fn conjugate(verb: &PtVocabularyVerb) -> PtVerbConjugations {
    let url = &format!("{}{}{}", CONJUGATION_ENDPOINT_STUB, verb.pt, ".html");

    let html = reqwest::get(url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    process_document(verb, html.as_str())
}

#[cfg(test)]
mod tests {
    use crate::lang::pt::{process_document, PtVocabularyVerb};

    #[tokio::test]
    async fn test_process_document() {
        let vocabulary_verb = PtVocabularyVerb {
            en: String::from("to be"),
            pt: String::from("ser"),
            comments: None,
        };

        let test_html = reqwest::get("https://conjugator.reverso.net/conjugation-portuguese-verb-ser.html")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let _verbs = process_document(&vocabulary_verb, test_html.as_str());
    }
}