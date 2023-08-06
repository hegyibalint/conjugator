use anki::notes::NoteId;
use anki::services::{CardsService, NotesService};

use crate::anki::AnkiHandler;
use crate::lang::pt::{PtVerbConjugations, PtVocabularyVerb};

impl AnkiHandler {

    pub(crate) fn has_verb(&mut self, p0: &PtVocabularyVerb) -> bool {
        let vec = self.collection.update_note(&p0.pt).unwrap();

        match vec.len() {
            0 => false,
            1 => true,
            _ => panic!("More than one note found for {}", p0.pt),
        }
    }

    pub(crate) fn add_verb(&mut self, verb: &PtVerbConjugations) {
        let deck = self.collection
            .get_or_create_normal_deck("Portuguese")
            .unwrap();

        let notetype = self.collection
            .get_notetype_by_name("Conjugator")
            .unwrap()
            .unwrap();

        let mut note = notetype.new_note();
        note.set_field(0, &verb.vocabulary.pt).unwrap();
        note.set_field(1, &verb.vocabulary.en).unwrap();
        note.set_field(2, "asd").unwrap();

        // Add the note to the collection, or update it if it already exists.
        let vec = self.collection.search_notes_unordered(&verb.vocabulary.pt).unwrap();

        match vec.len() {
            0 => {
                self.collection.add_note(
                    &mut note,
                    deck.id,
                ).unwrap();
            },
            1 => {
                note.id = NoteId::from(vec[0].0);
                self.collection.update_note(
                    &mut note,
                ).unwrap();
            },
            _ => {
                panic!("More than one note with the same vocabulary");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use anki::collection::CollectionBuilder;
    use anki::services::NotesService;
    use anki_proto::notes::NoteId;

    use crate::anki::AnkiHandler;
    use crate::lang::pt::{PtVerbConjugations, PtVocabularyVerb};

    #[test]
    fn test_add_verb_idempotence() {
        let collection = CollectionBuilder::default().build().unwrap();
        let mut anki_handler = AnkiHandler::from(collection);

        let verb_a = PtVerbConjugations {
            vocabulary: PtVocabularyVerb {
                pt: String::from("x"),
                en: String::from("a"),
                comments: None,
            },
            conjugations: HashMap::new(),
        };

        let verb_b = PtVerbConjugations {
            vocabulary: PtVocabularyVerb {
                pt: String::from("x"),
                en: String::from("b"),
                comments: None,
            },
            conjugations: HashMap::new(),
        };

        anki_handler.add_verb(&verb_a);
        let verb_a_search = anki_handler.collection.search_notes_unordered("x").unwrap();
        assert_eq!(verb_a_search.len(), 1);
        let note_id: NoteId =  NoteId {
            nid: verb_a_search[0].0,
        };

        let verb_a_note = anki_handler.collection.get_note(note_id.clone()).unwrap();
        assert_eq!(verb_a_note.fields[0], "x");
        assert_eq!(verb_a_note.fields[1], "a");

        anki_handler.add_verb(&verb_b);
        let verb_a_note = anki_handler.collection.get_note(note_id).unwrap();
        assert_eq!(verb_a_note.fields[0], "x");
        assert_eq!(verb_a_note.fields[1], "b");
    }
}