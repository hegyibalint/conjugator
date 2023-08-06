// CSV PROCESSING -------------------------------------------------------------
use crate::lang::pt::PtVocabularyVerb;

pub fn process_verb(fields: Vec<&str>) -> PtVocabularyVerb {
    PtVocabularyVerb {
        pt: fields[0].to_string(),
        en: fields[1].to_string(),
        comments: fields.get(2).map(|v| v.to_string()),
    }
}