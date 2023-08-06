use std::path::PathBuf;
use std::sync::Arc;

use anki::collection::{Collection, CollectionBuilder};
use anki::notetype::{CardTemplate, NoteField, Notetype};
use url::Url;

pub mod pt;

const TEMPLATE_QUESTION_FORMAT: &str = r#"
{{Source}}
<div style='font-family: "Liberation Sans"; font-size: 20px;'>{{AudioA}}</div>
"#;

const TEMPLATE_ANSWER_FORMAT: &str = r#"
{{Target}}

<hr id=answer>

{{Source}}

<hr id=extra>

{{Comment}}
{{#AudioB}}
<div style='font-family: "Liberation Sans"; font-size: 20px;'>{{AudioB}}</div>
{{/AudioB}}
"#;

const TEMPLATE_CSS: &str = r#"
.card {
    font-family: arial;
    font-size: 20px;
    text-align: center;
    color: black;
    background-color: white;
}
"#;

pub struct AnkiHandler {
    collection: Collection
}

impl From<Collection> for AnkiHandler {
    fn from(collection: Collection) -> Self {
        let mut anki_handler = AnkiHandler {
            collection,
        };
        anki_handler.get_or_create_notetype();

        anki_handler
    }
}

impl AnkiHandler {
    pub fn new(media_dir: PathBuf, _endpoint: Url) -> Self {
        let collection = CollectionBuilder::new("anki.db")
            .set_media_paths(media_dir, PathBuf::from("media.db"))
            .build()
            .unwrap();

        AnkiHandler {
            collection,
        }
    }

    pub fn prepare(&mut self) {
        create_notetype(&mut self.collection);
    }

    fn get_or_create_notetype(&mut self) -> Arc<Notetype> {
        match self.collection.get_notetype_by_name("Conjugator").unwrap() {
            Some(notetype) => notetype,
            None => create_notetype(&mut self.collection)
        }
    }

}

fn create_notetype(collection: &mut Collection) -> Arc<Notetype> {
    let mut notetype = Notetype::default();
    notetype.name = String::from("Conjugator");
    notetype.fields = vec![
        NoteField::new("Source"),
        NoteField::new("Target"),
        NoteField::new("Comment"),
        NoteField::new("AudioA"),
        NoteField::new("AudioB"),
    ];
    notetype.templates = vec![
        CardTemplate::new(
            "Standard Card",
            TEMPLATE_QUESTION_FORMAT,
            TEMPLATE_ANSWER_FORMAT
        )
    ];

    // By adding the note type, the ID field will be set
    collection.add_notetype(&mut notetype, false).unwrap();
    // We get the reference counted note type back from the collection
    collection.get_notetype(notetype.id).unwrap().unwrap()
}

#[cfg(test)]
mod tests {
    use anki::collection::CollectionBuilder;

    use crate::anki::create_notetype;

    #[test]
    fn test_notetype_creation() {
        let mut collection = CollectionBuilder::default().build().unwrap();
        let notetype = create_notetype(&mut collection);

        assert_eq!(notetype.fields.len(), 5);
        assert_eq!(notetype.fields[0].name, "Source");
        assert_eq!(notetype.fields[1].name, "Target");
        assert_eq!(notetype.fields[2].name, "Comment");
        assert_eq!(notetype.fields[3].name, "AudioA");
        assert_eq!(notetype.fields[4].name, "AudioB");
    }
}